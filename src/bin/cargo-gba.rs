#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(clippy::len_zero)]
use std::{
  env::args_os,
  ffi::{OsStr, OsString},
  fs::{create_dir, read_dir, DirEntry, File, Metadata},
  io::ErrorKind,
  path::{Path, PathBuf},
  process::{exit, Command},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main() {
  let mut args: Vec<_> = args_os().skip(1).map(OsString::into_string).collect();
  if args.contains(&Ok("--version".to_string())) {
    print_version();
    exit(0);
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
  }
}

fn gba_assemble() -> Result<(), String> {
  println!("Assembling...");
  let include_is_dir = Path::new("include").is_dir();
  let reader = read_dir("src").map_err(|e| match e.kind() {
    ErrorKind::NotFound => "Couldn't assemble: src directory not found".to_string(),
    _ => format!("Couldn't assemble: Unknown error: {}", e),
  })?;
  for result_dir_entry in reader {
    match result_dir_entry {
      Ok(dir_entry) => {
        if !dir_entry_is_file(&dir_entry).unwrap_or(false) {
          continue;
        }
        match PathBuf::from(dir_entry.file_name())
          .extension()
          .and_then(OsStr::to_str)
        {
          Some("s") => (),
          _ => continue,
        }
        let dir_entry_pathbuf = PathBuf::from(dir_entry.file_name());
        let file_stem = match dir_entry_pathbuf.file_stem() {
          Some(file_stem) => file_stem,
          None => continue,
        }
        .to_os_string();
        //
        let mut cmd_as = Command::new("arm-none-eabi-as");
        cmd_as.arg("-mcpu=arm7tdmi");
        cmd_as.arg("-mthumb-interwork");
        if include_is_dir {
          cmd_as.arg("-I");
          cmd_as.arg("include");
        }
        cmd_as.arg("-o");
        cmd_as.arg(format!(
          "{}",
          Path::new("target")
            .join(format!("{}.o", PathBuf::from(file_stem).display()))
            .display()
        ));
        cmd_as.arg(format!(
          "{}",
          Path::new("src").join(dir_entry.file_name()).display()
        ));
        println!("> {}", PathBuf::from(dir_entry.file_name()).display());
        let output = cmd_as
          .output()
          .map_err(|e| format!("Couldn't call the assembler! {}", e))?;
        if !output.status.success() {
          return Err(format!(
            "assembler error: {}",
            String::from_utf8_lossy(&output.stderr)
          ));
        }
      }
      Err(io_error) => {
        eprintln!(
          "Directory Entry IO Error (but will continue): {:?}",
          io_error
        );
      }
    }
  }
  Ok(())
}

fn dir_entry_is_file(dir_entry: &DirEntry) -> Result<bool, std::io::Error> {
  dir_entry.metadata().map(|metadata| metadata.is_file())
}

fn print_version() {
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

  println!("cargo-gba {} (binutils {})", VERSION, binutils_version);
}
