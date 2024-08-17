#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
        #[path = "generated/bindings_x86_64-unknown-linux-gnu.rs/bindings_x86_64-unknown-linux-gnu.rs"]
        mod bindings_linux;
        pub use bindings_linux::*;
    } else if #[cfg(all(target_arch = "x86_64", target_os = "windows"))] {
        #[path = "generated/bindings_x86_64-pc-windows-msvc.rs/bindings_x86_64-pc-windows-msvc.rs"]
        mod bindings_windows;
        pub use bindings_windows::*;
    } else if #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
        #[path = "generated/bindings_aarch64-apple-darwin.rs/bindings_aarch64-apple-darwin.rs"]
        mod bindings_macos;
        pub use bindings_macos::*;
    } else {
        compile_error!("This target is not supported");
    }
}
