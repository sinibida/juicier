use std::error::Error;

pub fn log_error(err: Box<dyn Error>) {
  println!("FAILED: {}", err.to_string());
}
