#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(clippy::len_zero)]
use std::{
  env::args_os,
  ffi::{OsStr, OsString},
  fs::{create_dir, create_dir_all, metadata, read_dir, DirEntry, File, Metadata},
  io::ErrorKind,
  iter::Extend,
  path::{Path, PathBuf},
  process::exit,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
struct EZCommand {
  pub program: OsString,
  pub args: Vec<OsString>,
}
impl EZCommand {
  pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
    Self {
      program: program.as_ref().to_os_string(),
      args: Vec::new(),
    }
  }

  pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) {
    self.args.push(arg.as_ref().to_os_string());
  }

  pub fn output_result(self) -> Result<Result<EZOutput, EZOutput>, std::io::Error> {
    let mut cmd = std::process::Command::new(&self.program);
    for arg in self.args.iter() {
      cmd.arg(arg);
    }
    cmd.output().map(|output| {
      if output.status.success() {
        Ok(EZOutput::from(output))
      } else {
        Err(EZOutput::from(output))
      }
    })
  }
}

#[derive(Debug, Clone)]
pub struct EZOutput {
  stdout: String,
  stderr: String,
}
impl From<std::process::Output> for EZOutput {
  fn from(value: std::process::Output) -> Self {
    Self {
      stdout: String::from_utf8_lossy(&value.stdout).into_owned(),
      stderr: String::from_utf8_lossy(&value.stderr).into_owned(),
    }
  }
}

pub fn main() {
  let mut args: Vec<_> = args_os().skip(1).map(OsString::into_string).collect();
  if args.get(0) == Some(&Ok("gba".to_string())) {
    args.remove(0).ok();
  }
  if args.contains(&Ok("--version".to_string())) {
    println!("{}", get_version_string());
    exit(0);
  }
  if args.contains(&Ok("--help".to_string())) {
    print_help_and_exit();
  }
  if args.len() > 0 {
    for arg in args {
      eprintln!("Unhandled Arg: {:?}", arg);
    }
  } else {
    create_dir("target").ok();
    gba_assemble().unwrap_or_else(|msg| {
      use std::io::Write;
      std::io::stdout().flush().ok();
      eprintln!("\nError during Assembling stage: {}", msg);
      exit(1);
    });
    gba_link().unwrap_or_else(|msg| {
      use std::io::Write;
      std::io::stdout().flush().ok();
      eprintln!("\nError during Linking stage: {}", msg);
      exit(1);
    });
  }
}

fn is_asm_file(pbr: &PathBuf) -> bool {
  pbr.is_file() && pbr.extension().and_then(OsStr::to_str) == Some("s")
}

fn gba_assemble() -> Result<(), String> {
  print!("as: ");
  let mut base_command = EZCommand::new("arm-none-eabi-as");
  base_command.arg("-mcpu=arm7tdmi");
  base_command.arg("-mthumb-interwork");
  base_command.arg("-I");
  base_command.arg("src");
  if Path::new("include").is_dir() {
    base_command.arg("-I");
    base_command.arg("include");
  }
  base_command.arg("-o");
  //
  let iter_chain = ReadDirSkipErrors::new("src")
    .filter(is_asm_file)
    .chain(ReadDirSkipErrors::new("src/bin").filter(is_asm_file))
    .chain(ReadDirSkipErrors::new("examples").filter(is_asm_file));
  for file_path in iter_chain {
    print!("{}, ", Path::new(file_path.file_stem().unwrap()).display());
    let mut this_command = base_command.clone();
    let out_file = {
      // TODO: maybe use
      // https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.strip_prefix
      // ?
      let mut out = Path::new("target").join(file_path.file_stem().unwrap());
      out.set_extension("o");
      out
    };
    this_command.arg(format!("{}", out_file.display()));
    this_command.arg(file_path);
    //
    this_command
      .output_result()
      .expect("Couldn't execute the assembler!")
      .map_err(|ez_out| ez_out.stderr)?;
  }
  println!();
  Ok(())
}

fn gba_link() -> Result<(), String> {
  print!("ld: ");
  let mut lib_files = vec![];
  for lib_file in ReadDirSkipErrors::new("src").filter(is_asm_file) {
    let mut lib_name = Path::new("target").join(lib_file.file_stem().unwrap());
    lib_name.set_extension("o");
    lib_files.push(lib_name);
  }
  let mut base_command = EZCommand::new("arm-none-eabi-ld");
  base_command.arg("--script");
  base_command.arg("gba_link_script.ld");
  base_command.arg("--output");
  for game in ReadDirSkipErrors::new("src/bin")
    .chain(ReadDirSkipErrors::new("examples"))
    .filter(is_asm_file)
    .map(|path|path.file_stem().unwrap().to_os_string())
  {
    let target_dir = Path::new("target");
    let mut o = target_dir.join(&game);
    o.set_extension("o");
    let mut elf = target_dir.join(&game);
    elf.set_extension("elf");
    print!("{}, ", Path::new(&game).display());
    //
    let mut this_command = base_command.clone();
    this_command.arg(format!("{}", elf.display()));
    this_command.arg(format!("{}", o.display()));
    for lib in lib_files.iter() {
      this_command.arg(format!("{}", lib.display()));
    }
    match this_command
      .output_result()
      .expect("Couldn't execute the linker!")
    {
      Ok(_) => (),
      Err(ez_output) => println!("Linker reported error: {}", ez_output.stderr),
    }
  }
  Ok(())
}

struct ReadDirSkipErrors {
  opt_rd: Option<std::fs::ReadDir>,
}
impl ReadDirSkipErrors {
  pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
    if base_dir.as_ref().is_dir() {
      match std::fs::read_dir(base_dir) {
        Ok(rd) => Self { opt_rd: Some(rd) },
        Err(_) => Self { opt_rd: None },
      }
    } else {
      Self { opt_rd: None }
    }
  }
}
impl core::iter::Iterator for ReadDirSkipErrors {
  type Item = PathBuf;
  fn next(&mut self) -> Option<PathBuf> {
    match self.opt_rd {
      None => None,
      Some(ref mut rd) => {
        loop {
          match rd.next() {
            Some(dir_entry_result) => {
              match dir_entry_result {
                Ok(dir_entry) => {
                  return Some(dir_entry.path());
                }
                Err(_) => {
                  // we don't care what the error was, just keep going.
                  continue;
                }
              }
            }
            None => {
              // our reader is exhausted
              self.opt_rd = None;
              return None;
            }
          }
        }
      }
    }
  }
}

fn get_version_string() -> String {
  // Note: all of the binutils in a given install should be the same version, so
  // we just call the assembler, because I guess it's alphabetical that way.
  let mut cmd = EZCommand::new("arm-none-eabi-as");
  cmd.arg("--version");
  let version_string: String = match cmd.output_result() {
    Ok(Ok(ez_output)) => ez_output
      .stdout
      .lines()
      .next()
      .and_then(|line| line.split_whitespace().last())
      .map(|v| v.to_string())
      .unwrap_or_else(|| "version could not be detected".to_string()),
    _ => "version could not be detected".to_string(),
  };

  format!("cargo-gba {} (binutils {})", VERSION, version_string)
}

fn print_help_and_exit() -> ! {
  println!("Just call the program with no args and it will assemble/link.");
  println!("* All `foo.s` files in `src/`, `src/bin/`, and `examples/` are assembled (into the `target/` dir)");
  println!("* The `foo.o` files from `src/bin/` and `examples/` are each linked into `foo.elf`");
  exit(0)
}
