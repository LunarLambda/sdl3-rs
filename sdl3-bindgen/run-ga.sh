#!/bin/bash

target=$1

if [ "$target" = "x86_64-unknown-linux-gnu" ]; then
    echo generating bindings_x86_64-unknown-linux-gnu.rs
    cargo run -- bindings_x86_64-unknown-linux-gnu.rs
elif [ "$target" = "x86_64-pc-windows-gnu" ]; then
    echo generating bindings_x86_64-pc-windows-gnu.rs
    BINDGEN_EXTRA_CLANG_ARGS='-target x86_64-pc-windows-gnu --sysroot /usr/x86_64-w64-mingw32' cargo run -- bindings_x86_64-pc-windows-gnu.rs
elif [ "$target" = "x86_64-pc-windows-msvc" ]; then
    echo generating bindings_x86_64-pc-windows-msvc.rs
    cargo run -- bindings_x86_64-pc-windows-msvc.rs
else
    echo 'unsupported platform'
fi
