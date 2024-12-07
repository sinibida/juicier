mod init;

use clap::{Parser, Subcommand};
use init::InitArgs;

/// Simple program to greet a person
#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
  /// Create a new cup to pour some OJ
  Init(InitArgs),
  /// Create a new cup to pour some OJ
  Add(InitArgs),
}

#[cfg(test)]
mod tests {

  use clap::Parser;

  use super::*;

  #[test]
  fn init_returns_init_args() {
    assert!(
      if let Commands::Init(_) = Cli::try_parse_from(["juic", "init"])
        .map_err(|err| err.to_string())
        .unwrap()
        .command
      {
        true
      } else {
        false
      }
    );
  }
}
