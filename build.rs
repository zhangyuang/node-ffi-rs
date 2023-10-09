extern crate napi_build;
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
  let env = std::env::var("env").unwrap_or_else(|_| String::from("production"));

  if env == "development" {
    println!("cargo:rustc-link-search=native=.");
    let bindings = bindgen::Builder::default()
      .header("./cpp/sum.h")
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      .allowlist_function(".*")
      .raw_line("#[link(name = \"sum\")]")
      .generate()
      .expect("Unable to generate bindings");
    bindings
      .write_to_file("./src/bindings.rs")
      .expect("Couldn't write bindings!");
  }

  napi_build::setup();
}
