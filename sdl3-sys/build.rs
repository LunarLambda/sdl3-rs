cfg_if::cfg_if! {
    if #[cfg(windows)] {
        // What the hell, bindgen?
        const REGEX: &str = r#"(.*[/\\])?sdl[/\\]include[/\\]SDL3[/\\].*"#;
    }
    else {
        const REGEX: &str = r#"(.*/)?sdl/include/SDL3/.*"#;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "static")] {
        const KIND: &str = "static";
    }
    else {
        const KIND: &str = "dylib";
    }
}

const LIB: &str = "SDL3";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cmake::Config::new("sdl");

    build
    .define("SDL_DISABLE_INSTALL_DOCS", "ON")
    .generator("Ninja");

    if cfg!(feature = "static") {
        build
        .define("SDL_SHARED", "OFF")
        .define("SDL_STATIC", "ON");
    }
    else {
        build
        .define("SDL_SHARED", "ON")
        .define("SDL_STATIC", "OFF");
    }

    let dst = build.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib={}={}", KIND, LIB);

    let _bindgen = bindgen::builder()
        .allowlist_file(REGEX)
        .clang_arg("-Isdl/include")
        .header("sdl/include/SDL3/SDL.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file("src/bindings.rs")
        .unwrap();
}
