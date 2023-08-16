const LIB: &str = "SDL3";

const TOP: &str = "#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dst = cmake::Config::new("sdl")
        .define("SDL_DISABLE_INSTALL_DOCS", "ON")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib={}", LIB);

    let _bindgen = bindgen::builder()
        .allowlist_file("sdl/include/SDL3/.*")
        .clang_arg("-Isdl/include")
        .header("sdl/include/SDL3/SDL.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .raw_line(TOP)
        .generate()
        .unwrap()
        .write_to_file("src/lib.rs")
        .unwrap();
}
