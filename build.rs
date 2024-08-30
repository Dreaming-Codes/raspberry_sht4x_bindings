extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let src_dir = "c_library";

    // Compile the C library
    cc::Build::new()
        .file(format!("{}/sensirion_common.c", src_dir))
        .file(format!("{}/sensirion_i2c_hal.c", src_dir))
        .file(format!("{}/sensirion_i2c.c", src_dir))
        .file(format!("{}/sht4x_i2c.c", src_dir))
        .include(src_dir)
        .compile("sensirion");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", src_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
