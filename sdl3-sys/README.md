# sdl3-sys

Heavily WIP bindings for SDL 3.0, which in itself is currently WIP.

- Requires CMake and Ninja for compiling SDL.
- Static linking only tested on Linux
- Compilation only tested on x86\_64 Linux and Windows (for now)

### Why Not \<other crate\>?

Many, many personal and impersonal gripes with the existing bindings and their build scripts.
Would rather just try and do it proper with minimal amounts of code from scratch.

### Windows (11) Instructions

- Install MSVC build tools and x86\_64-pc-windows-msvc Rust toolchain
- `winget install Kitware.CMake Ninja-build.Ninja LLVM.LLVM`
- `cargo build`

Running tests requires copying the DLL because /shrug/
