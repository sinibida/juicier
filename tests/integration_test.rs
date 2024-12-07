#[cfg(test)]
mod tests {
  use std::panic;

  fn setup() {
    // fs::create_dir("./e2e_tmp");
    println!("Setup!");
  }

  fn teardown() {
    println!("Teardown!");
  }

  fn run<T>(func: T) where T: Fn() -> () {
    setup();

    panic::set_hook(Box::new(|_| {
      teardown();
    }));

    func();

    let _ = panic::take_hook();
    teardown();
  }

  #[test]
  fn init_creates_project() {
    run(|| {
      assert!(false);
    })
  }
}
