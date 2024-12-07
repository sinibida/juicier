use clap::error::Error as ClapError;

/// Stub
pub enum Error {
  ParserError(ClapError)
}