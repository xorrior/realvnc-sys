

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search={}", "/usr/local/lib");
    println!("cargo:rustc-link-lib=vnc");
}