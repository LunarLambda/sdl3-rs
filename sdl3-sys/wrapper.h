// Bindgen chokes on some _Float16 _Complex definitions in clang's intrinsics headers.
// Preempt the headers from being included via their include guard.
//
// https://github.com/rust-lang/rust-bindgen/issues/2500#issuecomment-1640545912
#define __AVX512FP16INTRIN_H
#define __AVX512VLFP16INTRIN_H

// SDL.h includes pretty much everything we want, except syswm and vulkan
// The rest are expositional headers and bundled OpenGL headers that we don't care about.
#include <SDL3/SDL.h>
#include <SDL3/SDL_syswm.h>
#include <SDL3/SDL_vulkan.h>
