# sdl3-bindgen

The purpose of this crate is to externalize the generation of bindings
from sdl3-sys. Setting up bindgen as a dependency can be quite fickle,
and requires a fairly large LLVM installation. SDL3 does not use any build-time
configuration in its headers, so they can be pre-generated for every platform
without issue.

To use it, just run `cargo run`. The generated bindings will be written
to `bindings.rs`. For cross-compilation, you can make bindgen use a different
libclang via the `LIBCLANG_PATH` variable, and/or pass additional clang
arguments like `-target` or `--sysroot` via the `BINDGEN_EXTRA_CLANG_ARGS`
variable.

See [the bindgen documentation][bindgen-doc] for more information.

## License

sdl3-bindgen and SDL3 are licensed under the Zlib license.

[bindgen-doc]: https://docs.rs/bindgen/0.66.1/bindgen/struct.Builder.html#clang-arguments
