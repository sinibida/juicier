// use juicier::runner::*;

use juicier::{logger, runner::Runner};

fn main() {
  let runner = Runner::new();
  if let Err(err) = runner.run_cli() {
    logger::log_error(Box::new(err));
  }
}
