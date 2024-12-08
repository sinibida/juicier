use std::{error::Error, fs::File, io::Write};

use crate::parser::init::InitArgs;

const CUP_JSON_PATH: &str = "./cup.json";

pub fn run_init(_args: InitArgs) -> Result<(), Box<dyn Error>> {
  // Why does it automatically boxes the error?
  let mut file = File::create_new(CUP_JSON_PATH)?;

  file.write("Hello World!!!".as_bytes())?;

  Ok(())
}
