#[cfg(windows)]
// Bindgen/clang does not do path normalization, so we end up with stuff like
// <some prefix>\sdl/include\SDL3/<some header>
//
// TODO: Seems like it would generally be preferred to ship pregenerated bindings per-platform
// Since this A: gets rid of the ever-fickle bindgen dependency, as well as needing extra library
// headers for SDL(?), as well as letting us say "these bindings target version <x>" rather than
// just being whatever the downstream user has available.
const REGEX: &str = r#"(.*[/\\])?sdl[/\\]include[/\\]SDL3[/\\].*"#;

#[cfg(not(windows))]
const REGEX: &str = r#"(.*/)?sdl/include/SDL3/.*"#;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cmake::Config::new("sdl");

    build
    .define("SDL_DISABLE_INSTALL_DOCS", "ON")
    .define("SDL_SHARED", "ON")
    .define("SDL_STATIC", "OFF") // SDL discourages static linking (see docs/README-dynapi.md)
                                 // and pre-built binaries for Windows are only available as DLLs.
                                 // I know DLL search paths esp. in Rust are a pain, but maintaining
                                 // another binary build configuration is an even bigger pain.
    .generator("Ninja");         // A valiant effort at speeding up builds a little.

    let dst = build.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=SDL3");

    let _bindgen = bindgen::builder()
        .allowlist_file(REGEX)
        .clang_arg("-Isdl/include")
        // Bindgen chokes on _Float16 _Complex definitions in clang's intrinsics headers.
        // Disable them via preprocessor.
        .clang_arg("-D__AVX512VLFP16INTRIN_H")
        .clang_arg("-D__AVX512FP16INTRIN_H")
        // SDL's doc comments are inconsistent and don't translate to
        // rustdoc very well and cause things like spurious warnings and
        // build failures.
        .generate_comments(false)
        .header("sdl/include/SDL3/SDL.h")
        .header("sdl/include/SDL3/SDL_syswm.h")
        .header("sdl/include/SDL3/SDL_vulkan.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file("src/bindings.rs")
        .unwrap();
}
