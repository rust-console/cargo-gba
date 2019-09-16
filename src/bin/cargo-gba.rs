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
  process::{exit, Command},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
      eprintln!("{}", msg);
      exit(1);
    });
    gba_link().unwrap_or_else(|msg| {
      eprintln!("{}", msg);
      exit(1);
    });
  }
}

fn gba_assemble() -> Result<(), String> {
  println!("Assembling...");
  let include_is_dir = Path::new("include").is_dir();
  let file_paths = grab_file_paths_by_extension("src", "s", true);
  for file_path in file_paths {
    println!("> {}", file_path.display());
    let mut as_cmd = Command::new("arm-none-eabi-as");
    as_cmd.arg("-mcpu=arm7tdmi");
    as_cmd.arg("-mthumb-interwork");
    as_cmd.arg("-I");
    as_cmd.arg("src");
    if include_is_dir {
      as_cmd.arg("-I");
      as_cmd.arg("include");
    }
    as_cmd.arg("-o");
    let out_file = {
      // TODO: maybe use
      // https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.strip_prefix
      // ?
      let mut out = Path::new("target").join(file_path.file_stem().unwrap());
      out.set_extension("o");
      out
    };
    as_cmd.arg(format!("{}", out_file.display()));
    as_cmd.arg(file_path);
    //
    match as_cmd.output() {
      Ok(output) => {
        if output.status.success() {
          // nothing to do right now
        } else {
          return Err(format!(
            "Assembly failed! {}",
            String::from_utf8_lossy(&output.stderr)
          ));
        }
      }
      Err(err) => println!("ERROR: could not execute assembler. {}", err),
    }
  }
  Ok(())
}

fn gba_link() -> Result<(), String> {
  println!("Linking...");
  let mut ld_cmd = Command::new("arm-none-eabi-ld");
  ld_cmd.arg("--script");
  ld_cmd.arg("gba_link_script.ld");
  ld_cmd.arg("--output");
  let canonical_pwd = Path::new(".").canonicalize().unwrap();
  let project_name = Path::new(canonical_pwd.components().last().unwrap().as_os_str());
  let elf = Path::new("target").join(format!("{}.elf", project_name.display()));
  print!("> {}:", elf.display());
  ld_cmd.arg(format!("{}", elf.display()));
  let file_paths = grab_file_paths_by_extension("target", "o", false);
  for file_path in file_paths {
    print!(" {}", file_path.display());
    ld_cmd.arg(format!("{}", file_path.display()));
  }
  match ld_cmd.output() {
    Ok(output) => {
      if output.status.success() {
        // nothing to do right now
      } else {
        return Err(format!(
          "Linking failed! {}",
          String::from_utf8_lossy(&output.stderr)
        ));
      }
    }
    Err(err) => println!("ERROR: could not execute linker. {}", err),
  }
  Ok(())
}

fn grab_file_paths_by_extension<P: AsRef<Path>>(
  dir: P,
  ext: &str,
  recursive: bool,
) -> Vec<PathBuf> {
  let mut output = Vec::new();
  if dir.as_ref().is_dir() {
    if let Ok(read_dir) = read_dir(dir) {
      for dir_entry_result in read_dir {
        if let Ok(dir_entry) = dir_entry_result {
          let path_buf = dir_entry.path();
          if recursive && path_buf.is_dir() {
            output.extend(grab_file_paths_by_extension(path_buf, ext, true));
          } else if path_buf.is_file() && path_buf.extension().and_then(OsStr::to_str) == Some(ext)
          {
            output.push(path_buf);
          }
        }
      }
    }
  }
  output
}

fn get_version_string() -> String {
  // Note: all of the binutils in a given install should be the same version, so
  // we just call the assembler, because I guess it's alphabetical that way.
  let binutils_version = Command::new("arm-none-eabi-as")
    .arg("--version")
    .output()
    .ok()
    .and_then(|output| {
      if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
          .lines()
          .next()
          .and_then(|line| line.split_whitespace().last())
          .map(|v| v.to_string())
      } else {
        None
      }
    })
    .unwrap_or_else(|| "version could not be detected".to_string());

  format!("cargo-gba {} (binutils {})", VERSION, binutils_version)
}

fn print_help_and_exit() -> ! {
  println!("Just call the program with no args and it will assemble/link.");
  println!("* All foo.s files in src/ or subdirs are assembled into foo.o in target/");
  println!("* All foo.o files in target/ are linked into PROJECT.elf");
  exit(0)
}
