extern crate napi_build;

fn main() {
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
  napi_build::setup();
}
