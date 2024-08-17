#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg_attr(all(target_arch = "x86_64", target_os = "linux"), path = "bindings/x86_64-unknown-linux-gnu.rs")]
#[cfg_attr(all(target_arch = "x86_64", target_os = "windows"), path = "bindings/x86_64-pc-windows-msvc.rs")]
#[cfg_attr(all(target_arch = "aarch64", target_os = "macos"), path = "bindings/aarch64-apple-darwin.rs")]
mod bindings;
pub use bindings::*;
