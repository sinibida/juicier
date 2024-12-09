use clap::error::Error as ClapError;
use std::{error::Error as NativeError, fmt::Display};

/// Stub
#[derive(Debug)]
pub enum Error {
  ParserError(ClapError),
  ExecutionError(Box<dyn NativeError>),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::ParserError(error) => Display::fmt(error, f),
      Error::ExecutionError(error) => Display::fmt(error, f),
    }
  }
}

impl NativeError for Error {}
