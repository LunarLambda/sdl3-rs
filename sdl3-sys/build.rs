fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    cmake_build();
    run_bindgen();
}


fn cmake_build() {
    let dst = cmake::Config::new("sdl")
        .define("SDL_DISABLE_INSTALL_DOCS", "ON")
        .define("SDL_TEST_LIBRARY", "OFF")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=SDL3");
}

// FIXME: replace with pre-generated bindings. This is possible because the SDL3 headers have no
// build-time logic, just per-platform logic, so it's relatively straightforward to generate
// bindings for every platform and shove them in src/.
fn run_bindgen() {
    bindgen::builder()
        .allowlist_file(r#"(.*[/\\])?SDL3[/\\].*"#)
        .blocklist_file(r#"(.*[/\\])?SDL_platform_defines.h"#) // Don't need these
        .blocklist_type("SDL_DUMMY_ENUM")
        // SDL's doc comments are inconsistent and don't translate to
        // rustdoc very well and cause things like spurious warnings and
        // build failures.
        .generate_comments(false)
        .generate_cstr(true)
        .opaque_type("va_list")
        .prepend_enum_name(false)
        .clang_arg("-Isdl/include")
        // Bindgen chokes on _Float16 _Complex definitions in clang's intrinsics headers.
        // Disable them via preprocessor.
        .clang_arg("-D__AVX512FP16INTRIN_H")
        .clang_arg("-D__AVX512VLFP16INTRIN_H")
        // SDL.h includes everything we want except syswm and vulkan. The rest are expositional
        // headers and bundled OpenGL stuff we don't care about.
        .header("sdl/include/SDL3/SDL.h")
        .header("sdl/include/SDL3/SDL_syswm.h")
        .header("sdl/include/SDL3/SDL_vulkan.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file("src/bindings.rs")
        .unwrap();
}
