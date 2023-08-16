// What the hell, bindgen?
#[cfg(windows)]
const REGEX: &str = r#"(.*[/\\])?sdl[/\\]include[/\\]SDL3[/\\].*"#;
#[cfg(not(windows))]
const REGEX: &str = r#"(.*/)?sdl/include/SDL3/.*"#;

const LIB: &str = "SDL3";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dst = cmake::Config::new("sdl")
        .define("SDL_DISABLE_INSTALL_DOCS", "ON")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib={}", LIB);

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
