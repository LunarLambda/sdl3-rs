#!/bin/bash

set -e

target="${1:?missing target}"

if [ "$target" = "x86_64-pc-windows-gnu" ]; then
    export BINDGEN_EXTRA_CLANG_ARGS='-target x86_64-pc-windows-gnu --sysroot /usr/x86_64-w64-mingw32'
fi

echo "generating $target.rs"
cargo run -- "$target.rs"
