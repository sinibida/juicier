mod error;
mod init;

use clap::Parser;
use error::Error;
use init::run_init;

use crate::parser::Cli;
use crate::parser::Commands;

pub struct Runner {}
impl Runner {
  /// Stub
  pub fn new() -> Self {
    return Runner {};
  }

  fn run_command_arg(&self, command: Commands) -> Result<(), Error> {
    match command {
      Commands::Init(args) => run_init(args),
      _ => todo!(),
    }.map_err(|err| Error::ExecutionError(err))
  }

  /// Stub
  pub fn run_command(self: &Self, command: String) -> Result<(), Error> {
    let parsed =
      Cli::try_parse_from(command.split_whitespace()).map_err(|err| Error::ParserError(err))?;

    self.run_command_arg(parsed.command)
  }

  pub fn run_cli(self: &Self) -> Result<(), Error> {
    let parsed = Cli::try_parse().map_err(|err| Error::ParserError(err))?;

    self.run_command_arg(parsed.command)
  }
}
