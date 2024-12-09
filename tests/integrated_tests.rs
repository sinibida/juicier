#[cfg(test)]
mod tests {
  use std::fs;

  use juicier::{runner::Runner, util::test::TempFolderContext};

  /// running `juic init` creates 'cup', where various oj environment lives.
  #[test]
  fn init_creates_cup() {
    let cd = TempFolderContext::new().unwrap();

    let runner = Runner::new();
    let result = runner.run_command("juic init".to_string());
    assert!(matches!(result, Ok(())));
    assert!(fs::exists("./cup.json").is_ok_and(|x| x));

    cd.close();
  }
}
