mod error;
mod init;

use clap::Parser;
use error::Error;
use init::run_init;

use crate::logger::log_error;
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
      Commands::Init(args) => {
        if let Err(err) = run_init(args) {
          log_error(err);
        }
      }
      _ => panic!("unimplemented"),
    }

    Ok(())
  }
}
