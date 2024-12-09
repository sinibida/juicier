mod e2e_pwd;

use e2e_pwd::WithTestFolder;

#[cfg(test)]
mod tests {

  use std::fs;

  use super::WithTestFolder;

  use juicier::runner::Runner;

  /// running `juic init` creates 'cup', where various oj environment lives.
  #[test]
  fn init_creates_cup() {
    WithTestFolder {
      clean: true,
      ..Default::default()
    }
    .run(|| {
      let runner = Runner::new();
      let result = runner.run_command("juic init".to_string());
      assert!(!matches!(result, Err(_)));
      assert!(fs::exists("./cup.json").is_ok_and(|x| x));
    })
  }
}
