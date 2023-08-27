#!/bin/sh

UNAME=$(uname)

if [ "$UNAME" = "Linux" ]; then
    echo generating bindings_linux.rs
    cargo run -q -- bindings_linux.rs
    if [ -d '/usr/x86_64-w64-mingw32' ]; then
        echo generating bindings_windows.rs
        BINDGEN_EXTRA_CLANG_ARGS='-target x86_64-pc-windows-gnu --sysroot /usr/x86_64-w64-mingw32' cargo run -q -- bindings_windows.rs
    fi
else
    echo 'unsupported platform'
fi
