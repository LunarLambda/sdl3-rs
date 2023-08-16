fn main() {
    let dst = cmake::Config::new("sdl")
        .define("SDL_DISABLE_INSTALL_DOCS", "ON")
        .generator("Ninja")
        .build();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=SDL3");

    let _bindgen = bindgen::builder()
        .clang_arg("-Isdl/include")
        .header("sdl/include/SDL3/SDL.h")
        .allowlist_file("sdl/include/SDL3/.*")
        .raw_line("#![allow(non_camel_case_types)]\n#![allow(non_upper_case_globals)]\n#![allow(non_snake_case)]")
        .generate()
        .unwrap()
        .write_to_file("src/lib.rs")
        .unwrap();
}
