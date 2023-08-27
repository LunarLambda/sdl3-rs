fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(feature = "check") {
        return;
    }

    if cfg!(feature = "bundled") {
        cmake_build();
    } else {
        run_pkg_config();
    }
}

fn run_pkg_config() {
    pkg_config::Config::new().probe("sdl3").unwrap();
}

fn cmake_build() {
    let dst = cmake::Config::new("sdl")
        .define("SDL_DISABLE_INSTALL_DOCS", "ON")
        .define("SDL_TEST_LIBRARY", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=SDL3");
}
