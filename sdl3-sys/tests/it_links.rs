use sdl3_sys::*;

#[test]
fn it_links() {
    unsafe {
       SDL_Init(0);
       SDL_Quit();
    }
}

#[test]
fn its_3_0() {
    unsafe {
        let mut v = SDL_version { major: 0, minor: 0, patch: 0 };

        SDL_GetVersion(&mut v as _);

        assert_eq!(v.major, 3);
    }
}
