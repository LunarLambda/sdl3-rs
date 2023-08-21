fn main() {
    bindgen::builder()
        .allowlist_file(r#"(.*[/\\])?SDL3[/\\].*"#)
        .blocklist_file(r#"(.*[/\\])?SDL_platform_defines.h"#) // Don't need these
        .blocklist_type("SDL_DUMMY_ENUM")
        // Drop the va_list stuff because it's not useful in current Rust
        .blocklist_type(".*va_list.*")
        .blocklist_function("SDL_LogMessageV")
        .blocklist_function(r#"SDL_v\w+(scanf|printf)"#)
        // SDL's doc comments are inconsistent and don't translate to
        // rustdoc very well and cause things like spurious warnings and
        // build failures.
        .generate_comments(false)
        .generate_cstr(true)
        .prepend_enum_name(false)
        .sort_semantically(true)
        .clang_arg("-Iheaders")
        .header("wrapper.h")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file("bindings.rs")
        .unwrap();
}
