#[cfg(test)]
mod tests {
  use std::env;

  use assert_cmd::Command;
  use assert_fs::prelude::PathChild;
  use juicier::util::test::TempFolderContext;

  /// running `juic init` creates 'cup', where various oj environment lives.
  #[test]
  fn init_creates_cup() {
    let cd = TempFolderContext::new().unwrap();
    let temp_dir = cd.get_temp_dir();

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    let mut cmd = &mut Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd = cmd.arg("init");

    cmd.assert().success();

    assert!(temp_dir.child("cup.json").exists());

    cd.close();
  }
}
