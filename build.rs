extern crate bindgen;
extern crate napi_build;

fn main() {
  println!("cargo:rustc-link-lib=dylib=liblz4");
  println!("cargo:rustc-link-search=native=./library");
  println!("cargo:rerun-if-changed=./src/lib.rs");

  let bindings = bindgen::Builder::default()
    .header("./library/coreso.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file("./src/coreso_bindings.rs")
    .expect("Couldn't write bindings!");

  napi_build::setup();
}
