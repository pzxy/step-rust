extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html
    // Tell cargo to look for shared libraries in the specified directory
    // 绝对路径
    println!(concat!(
        "cargo:rustc-link-search=",
        env!("CARGO_MANIFEST_DIR"),
        "/src/pcre/lib"
    ));
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=pcre2-8");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/pcre/include/pcre2.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/pcre/include/pcre2.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
