#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg_attr(
    any(target_os = "macos"),
    link(name = "libvncsdk", kind = "dylib")
)]
extern "C" {

}

pub mod init;