fn main() {
    let out = std::env::args().nth(1).unwrap_or(String::from("bindings.rs"));

    bindgen::builder()

    // allow only the SDL3 headers to avoid unnecessary libc definitions
    .allowlist_file(r#"(.*[/\\])?SDL3[/\\].*\.h"#)

    // only used for platform detection in C. use cfg(...) instead
    .blocklist_file(r#"(.*[/\\])?SDL_platform_defines.h"#)

    // only used for static assertions
    .blocklist_type("SDL_DUMMY_ENUM")

    // va_list functions aren't really usable in Rust right now
    .blocklist_type(".*va_list.*")
    .blocklist_function("SDL_LogMessageV")
    .blocklist_function(r#"SDL_v\w+(scanf|printf)"#)

    // forward declaration used in SDL_main.h, not applicable to rust
    .blocklist_function("SDL_main")

    // pulled in on windows targets for SDL_TriggerBreakpoint, which we don't expose
    .blocklist_function("__debugbreak")

    // used by SDL_assert.h for silencing warnings in their macros
    .blocklist_item("SDL_NULL_WHILE_LOOP_CONDITION")

    // SDL's documentation comments are inconsistent and don't translate to
    // rustdoc very well, causing warnings and build failures due to
    // being misinterpreted as doctests. best to use the official wiki instead
    .generate_comments(false)

    // used mostly for the hints API
    // more semantically correct and saves some typecasting
    .generate_cstr(true)

    // SDL's enum naming is fairly good, so prepending the enum name just
    // causes the identifiers to be longer than they need to be
    .prepend_enum_name(false)

    // more orderly and may generate more diffable files
    .sort_semantically(true)

    .clang_arg("-Iheaders")
    .header("wrapper.h")
    .generate()
    .unwrap()
    .write_to_file(out)
    .unwrap();
}
