use failure::err_msg;
use sdl::video::Surface;
use sdl::video::ll::{SDL_LoadBMP_RW, SDL_RWFromConstMem};
use error::*;

mod cpp {
    use libc::{c_int, c_double};
    use sdl::video::ll::SDL_Surface;

    extern "C" {
        pub fn adjust_brightness(image: * mut SDL_Surface, k: c_double, transparent: c_int) -> * mut SDL_Surface;
        pub fn adjust_brightness_pixel(image: * mut SDL_Surface, x: c_int, y: c_int, k: c_double);
    }
}

pub fn adjust_brightness(image: &Surface, k: f64, transparent: bool) -> Surface {
    let s = unsafe { cpp::adjust_brightness(image.raw, k, if transparent { 1 } else { 0 }) };
    Surface { raw: s, owned: true }
}

pub fn adjust_brightness_pixel(image: &mut Surface, x: u16, y: u16, k: f64) {
    unsafe { cpp::adjust_brightness_pixel(image.raw, x as i32, y as i32, k) };
}

pub fn load_image(data: &[u8]) -> Result<Surface> {
    let surface = unsafe {
        let op = SDL_RWFromConstMem(data.as_ptr() as * const ::libc::c_void, data.len() as i32);
        Surface { raw: SDL_LoadBMP_RW(op, 0), owned: true }
    };
    let surface = surface.display_format().map_err(err_msg)?;
    Ok(surface)
}
