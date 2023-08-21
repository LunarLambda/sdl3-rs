fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    cmake_build();
    run_bindgen();
}

fn cmake_build() {
    let dst = cmake::Config::new("sdl")
        .define("SDL_DISABLE_INSTALL_DOCS", "ON")
        .define("SDL_TEST_LIBRARY", "OFF")
        //.generator("Ninja") // CMAKE_GENERATOR environment variable
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
        // drop the va_list stuff because it's not useful
        .blocklist_type(".*va_list")
        .blocklist_function("SDL_LogMessageV")
        .blocklist_function(r#"SDL_v\w+(scanf|printf)"#)
        .prepend_enum_name(false)
        .sort_semantically(true)
        .clang_arg("-Isdl/include")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file("src/bindings.rs")
        .unwrap();
}
