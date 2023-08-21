#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod bindings_linux;
        pub use bindings_linux::*;
    } else {
        compile_error!("no bindings available for the current platform")
    }
}
