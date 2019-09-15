use std::{env::args_os, process::Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main() {
  println!("this is cargo-gba");
  for arg_string in args_os().map(|arg_os| arg_os.to_string_lossy().into_owned()) {
    println!("{}", arg_string);
    if arg_string == "--version" {
      println!(
        "cargo-gba {} (binutils {})",
        VERSION,
        get_binutils_version().unwrap_or_else(|_| "could not be detected".to_string())
      );
    }
  }
}

pub enum GetBinutilsVersionError {
  CouldNotExecuteCommand(std::io::Error),
  CommandExecutionFailed { stdout: Vec<u8>, stderr: Vec<u8> },
  CouldNotParseVersionOutput,
}

fn get_binutils_version() -> Result<String, GetBinutilsVersionError> {
  Command::new("arm-none-eabi-as")
    .arg("--version")
    .output()
    .map_err(GetBinutilsVersionError::CouldNotExecuteCommand)
    .and_then(|output| {
      if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
          .lines()
          .next()
          .and_then(|line| line.split_whitespace().last())
          .map(|v| v.to_string())
          .ok_or(GetBinutilsVersionError::CouldNotParseVersionOutput)
      } else {
        Err(GetBinutilsVersionError::CommandExecutionFailed {
          stdout: output.stdout,
          stderr: output.stderr,
        })
      }
    })
}
