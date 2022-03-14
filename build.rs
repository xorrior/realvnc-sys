extern crate bindgen;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search={}", "/usr/local/lib");
    println!("cargo:rustc-link-lib=vnc");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let generated = bindings.to_string();

    let out_path = out_path.join("bindings.rs");
    let mut out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_path)
        .expect("Unable to open file");

    out_file.write_all(generated.as_bytes())
        .expect("Unable to write bindings file");
}