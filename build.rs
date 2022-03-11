extern crate bindgen;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=all={}", Path::new(&dir).join("lib").display());
    println!("cargo:rustc-link-lib=libvncsdk");
    println!("cargo:rerun-if-changed=wrapper.h");

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