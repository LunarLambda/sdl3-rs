use sdl3_sys::*;

#[test]
fn it_links() {
    unsafe {
       SDL_Init(0);
       SDL_Quit();
    }
}
