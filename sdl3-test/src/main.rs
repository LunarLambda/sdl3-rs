use std::os::raw::*;

// This is the main() the user would write
fn user_main() -> std::io::Result<()> {
    println!("Hello");
    std::fs::File::open("doesntexist").map(drop)
}

// This 'main' is what we would generate, using e.g. a macro.
fn main() -> /* pasted */ std::io::Result<()> {
    use std::mem::MaybeUninit;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type R = std::thread::Result</* pasted */ std::io::Result<()>>;

    let mut return_slot = MaybeUninit::<R>::uninit();

    static RETURN_PTR: AtomicPtr<R> = AtomicPtr::new(std::ptr::null_mut());

    RETURN_PTR.store(return_slot.as_mut_ptr(), Ordering::SeqCst);

    // Magic C-ABI shim that calls user_main and smuggles the return value out of it
    // So that the real main() can propagate any panic/return value
    // All so that user_main() can behave exactly like main() normally would.
    unsafe extern "C" fn sdl_main_shim(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
        let ptr = RETURN_PTR.load(Ordering::SeqCst);
        let result = std::panic::catch_unwind(user_main);
        unsafe { ptr.write(result); }
        return 0;
    }

    unsafe {
        // Run main
        // Calling this function is very very important but also extremely hard.
        sdl3_sys::SDL_RunApp(0, std::ptr::null_mut(), Some(sdl_main_shim), std::ptr::null_mut());
    }

    // SAFETY: This was initialised by sdl_main_shim by means of RETURN_PTR.
    let result = unsafe { return_slot.assume_init() };

    match result {
        Ok(r) => r,
        Err(e) => std::panic::resume_unwind(e)
    }
}
