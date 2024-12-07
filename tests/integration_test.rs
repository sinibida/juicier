#[cfg(test)]
mod tests {
  use std::env;
  use std::fs;
  use std::panic;
  use std::path::Path;

  const TEST_PATH: &'static str = "./e2e_temp";

  struct Context {
    pwd: String,
  }

  fn get_path() -> String {
    Path::new(".").join(TEST_PATH).to_str().unwrap().to_owned()
  }

  fn setup(context: &mut Context) {
    fs::create_dir(get_path())
      .map_err(|err| format!("failed creating directory: {}", err.kind()))
      .unwrap();

    context.pwd = env::current_dir().unwrap().to_str().unwrap().to_string();
    env::set_current_dir(get_path())
      .map_err(|err| format!("failed cd: {}", err.kind()))
      .unwrap();
  }

  fn teardown(context: &Context) {
    env::set_current_dir(Path::new(&context.pwd))
      .map_err(|err| format!("failed cd: {}", err.kind()))
      .unwrap();
    fs::remove_dir_all(get_path())
      .map_err(|err| format!("failed removing directory: {}", err.kind()))
      .unwrap();
  }

  fn run<T>(func: T)
  where
    T: Fn() -> () + panic::RefUnwindSafe,
  {
    let mut context = Context { pwd: String::new() };
    setup(&mut context);

    let result = panic::catch_unwind(|| {
      func();
    });

    teardown(&context);

    if let Err(err) = result {
      panic::resume_unwind(err)
    }
  }

  #[test]
  fn init_creates_project() {
    run(|| {
      assert!(true);
    })
  }
}
