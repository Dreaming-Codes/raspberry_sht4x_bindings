extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Compile the C library
    cc::Build::new()
        .file("c_library/sht4x_i2c.c")
        .include("c_library/..")
        .compile("sht4x");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("c_library/sht4x_i2c.h")
        .clang_arg("-Ic_library/..")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
