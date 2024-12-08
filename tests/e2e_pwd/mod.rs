use std::env;
use std::fs;
use std::panic;
use std::path::Path;

const TEST_PATH: &'static str = "./e2e_temp";

struct Context {
  path: String,
  pwd: String,
}

fn get_path_inner(retry: u16) -> String {
  let timestamp = chrono::Local::now().timestamp();

  let mut filename = format!("{TEST_PATH}-{timestamp}");
  if retry > 0 {
    filename = format!("{filename}-{retry}");
  }

  Path::new(".").join(filename).to_str().unwrap().to_owned()
}

fn get_path() -> String {
  let mut retry = 0;
  loop {
    let path = get_path_inner(retry);
    if fs::exists(&path).is_ok_and(|x| x) {
      retry += 1;
      continue;
    } else {
      return path.as_str().to_owned();
    }
  }
}

fn setup(context: &mut Context) {
  context.path = get_path();
  fs::create_dir(&context.path)
    .map_err(|err| format!("failed creating directory: {}", err.kind()))
    .unwrap();

  context.pwd = env::current_dir().unwrap().to_str().unwrap().to_string();
  env::set_current_dir(&context.path)
    .map_err(|err| format!("failed cd: {}", err.kind()))
    .unwrap();
}

fn teardown(context: &Context, clean: bool) {
  env::set_current_dir(Path::new(&context.pwd))
    .map_err(|err| format!("failed cd: {}", err.kind()))
    .unwrap();
  if clean {
    fs::remove_dir_all(&context.path)
      .map_err(|err| format!("failed removing directory: {}", err.kind()))
      .unwrap();
  }
}

pub struct WithTestFolder {
  pub clean: bool,
}

impl Default for WithTestFolder {
  fn default() -> Self {
    Self { clean: true }
  }
}

impl WithTestFolder {
  /// Runs provided function in test folder & pwd.
  /// It will be run inside `TEST_PATH` path, defined as a constant inside the source code.
  /// (`"./e2e_temp"`)
  pub fn run<T>(self: &Self, func: T)
  where
    T: Fn() -> () + panic::RefUnwindSafe,
  {
    let mut context = Context {
      pwd: String::new(),
      path: String::new(),
    };

    setup(&mut context);
    let result = panic::catch_unwind(|| {
      func();
    });
    teardown(&context, self.clean);

    if let Err(err) = result {
      panic::resume_unwind(err)
    }
  }
}
