mod error;

use clap::Parser;
use error::Error;

use crate::parser::Cli;
use crate::parser::Commands;

pub struct Runner {}
impl Runner {
  /// Stub
  pub fn new() -> Self {
    return Runner {};
  }

  /// Stub
  pub fn run_command(self: &Self, command: String) -> Result<(), Error> {
    let parsed =
      Cli::try_parse_from(command.split_whitespace()).map_err(|err| Error::ParserError(err))?;

    match parsed.command {
      Commands::Init(_args) => {
        // Stub
        println!("Init Run!")
      },
      _ => panic!("unimplemented")
    }

    Ok(())
  }
}
