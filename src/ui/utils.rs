use sdl::video::Surface;

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
