// Bindgen chokes on some _Float16 _Complex definitions in clang's intrinsics headers.
// Preempt the headers from being included via their include guard.
//
// https://github.com/rust-lang/rust-bindgen/issues/2500#issuecomment-1640545912
#define __AVX512FP16INTRIN_H
#define __AVX512VLFP16INTRIN_H

// SDL.h includes pretty much everything we want, except vulkan
// The rest are expositional headers and bundled OpenGL headers that we don't care about.
#include <SDL3/SDL.h>
#include <SDL3/SDL_vulkan.h>

// SDL provides initialization code for several platforms in the form of redefining main(),
// and calling SDL_RunApp from the platform's entry function (e.g. WinMain), which in turn
// performs the necessary setup and calls SDL_SetMainReady followed by the user's main function.
//
// In Rust, this is a lot more complicated, because for example things like WinMain are already provided
// by the language runtime (std). Maybe a proc macro in the spirit of something like #[tokio::main]
// could handle this boilerplate for the platforms that need it, but at any rate, the functions should
// be exposed by the bindings. The macro SDL_MAIN_HANDLED ensures that the header only provides declarations.
//
// Windows (non-WinRT/UWP), Linux, and MacOS at any rate do not need any special setup. Rust handles everything.
#define SDL_MAIN_HANDLED
#include <SDL3/SDL_main.h>
