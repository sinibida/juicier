mod e2e_pwd;

use e2e_pwd::run_inside_test_path;

#[cfg(test)]
mod tests {

  use std::fs;

  use super::run_inside_test_path;

  use juicier::runner::Runner;

  /// running `juic init` creates 'cup', where various oj environment lives.
  #[test]
  fn init_creates_cup() {
    run_inside_test_path(|| {
      let runner = Runner::new();
      let _ = runner.run_command("init".to_string());
      assert!(fs::exists("./cup.json").is_ok_and(|x| x));
    })
  }
}
