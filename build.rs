use std::env;
use std::path::PathBuf;

fn main() {
    // Try to guess where librrd.a / librrd.so live
    let library = if cfg!(target_os = "macos") {
        "/opt/homebrew/lib"
    } else {
        "/usr/lib/"
    };

    println!("cargo:rustc-link-search={}", library);

    // Tell cargo to tell rustc to link the rrd shared library
    println!("cargo:rustc-link-lib=rrd");

    // Try to determine which wrapper.h to include
    let headerfile = if cfg!(target_os = "macos") {
        "wrapper-osx.h"
    } else {
        "wrapper-linux.h"
    };

    // Build bindings with bindgen::Builder
    let bindings = bindgen::Builder::default()
        .header(headerfile)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
