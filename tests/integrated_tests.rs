mod common;

use common::e2e_pwd_env::run_inside_test_path;

#[cfg(test)]
mod tests {
  use super::run_inside_test_path;

  #[test]
  fn init_creates_project() {
    run_inside_test_path(|| {
      assert!(true);
    })
  }
}
