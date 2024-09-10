extern crate napi_build;
macro_rules! p {
  ($($tokens: tt)*) => {
      println!("cargo:warning={}", format!($($tokens)*))
  }
}

fn main() {
  napi_build::setup();
}
