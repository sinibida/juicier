mod error;

use error::Error;

pub struct Runner {}
impl Runner {
  /// Stub
  pub fn new() -> Self {
    return Runner {};
  }

  /// Stub
  pub fn run_command(self: &Self, command: String) -> Result<(), Error> {
    Ok(())
  }
}
