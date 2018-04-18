use failure::err_msg;
use sdl::video::{Surface, SurfaceFlag};
use sdl::video::ll::{SDL_LoadBMP_RW, SDL_RWFromConstMem};
use sdl2::rect::Rect;
use error::*;

pub fn rect2_to_rect(rect: Rect) -> ::sdl::Rect {
    ::sdl::Rect::new(rect.left() as i16, rect.top() as i16, rect.width() as u16, rect.height() as u16)
}

pub fn rect_to_rect2(rect: ::sdl::Rect) -> Rect {
    Rect::new(rect.x as i32, rect.y as i32, rect.w as u32, rect.h as u32)
}

mod cpp {
    use libc::{c_double};
    use sdl::video::ll::SDL_Surface;

    extern "C" {
        pub fn adjust_brightness(image: * mut SDL_Surface, k: c_double) -> * mut SDL_Surface;
    }
}

pub fn adjust_brightness(image: &Surface, k: f64) -> Surface {
    let s = unsafe { cpp::adjust_brightness(image.raw, k) };
    Surface { raw: s, owned: true }
}

pub fn load_image(data: &[u8]) -> Result<Surface> {
    let surface = unsafe {
        let op = SDL_RWFromConstMem(data.as_ptr() as * const ::libc::c_void, data.len() as i32);
        Surface { raw: SDL_LoadBMP_RW(op, 0), owned: true }
    };
    let surface = surface.display_format().map_err(err_msg)?;
    surface.set_color_key(&[SurfaceFlag::SrcColorKey], ::sdl::video::Color::RGBA(255, 255, 255, 255));
    Ok(surface)
}
