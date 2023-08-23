use sdl3_sys::*;

fn main() {
    unsafe {
        assert_eq!(SDL_Init(SDL_INIT_VIDEO as _), 0);

        let w = SDL_CreateWindow(b"Hello SDL3\0".as_ptr() as _, 480, 480, 0);

        assert_ne!(w, std::ptr::null_mut());

        let r = SDL_CreateRenderer(w, std::ptr::null(), 0);

        assert_ne!(r, std::ptr::null_mut());

        SDL_SetRenderDrawColor(r, 0, 127, 255, 255);
        SDL_RenderClear(r);
        SDL_RenderPresent(r);

        SDL_Delay(5000);

        SDL_Quit();
    }
}
