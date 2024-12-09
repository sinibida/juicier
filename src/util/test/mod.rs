use std::{env, error::Error, path::PathBuf};

use assert_fs::TempDir;

pub struct TempFolderContext {
  stashed_path: PathBuf,
  temp_dir: TempDir,
}

impl Drop for TempFolderContext {
  fn drop(&mut self) {
    env::set_current_dir(&self.stashed_path).expect("cd return failed.");
  }
}

impl TempFolderContext {
  fn new_inner(persistent: bool) -> Result<Self, Box<dyn Error>> {
    let ret: Self = Self {
      stashed_path: env::current_dir()?,
      temp_dir: TempDir::new()?.into_persistent_if(persistent),
    };

    env::set_current_dir(ret.temp_dir.path())?;

    Ok(ret)
  }

  pub fn new() -> Result<Self, Box<dyn Error>> {
    Self::new_inner(false)
  }
  /// Creates cd context, but retains the temp folder on end
  pub fn new_persistent() -> Result<Self, Box<dyn Error>> {
    Self::new_inner(true)
  }

  pub fn close(self) {}
}
