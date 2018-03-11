use failure::err_msg;
use sdl::video::{Surface, SurfaceFlag};
use sdl::video::ll::{SDL_LoadBMP_RW, SDL_RWFromConstMem};
use sdl::video::ll::SDL_Surface;
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use error::*;

unsafe fn surface2_to_surface(surface: &sdl2::surface::Surface) -> Surface {
    Surface { raw: surface.raw() as * mut SDL_Surface, owned: false }
}

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

pub fn tiled_image(data: &[u8], width: u16, height: u16) -> Result<Surface> {
    let win = Surface::new(&[SurfaceFlag::SWSurface], width as isize, height as isize, 32, 0, 0, 0, 0).map_err(err_msg)?;

    let tile = load_image(data)?;

    let tile_width = tile.get_width();
    let tile_height = tile.get_height();

    let cw = (width + tile_width - 1) / tile_width;
    let ch = (height + tile_height - 1) / tile_height;

    for j in 0..ch {
        for i in 0..cw {
            win.blit_at(&tile, (i * tile_width) as i16, (j * tile_height) as i16);
        }
    }

    let image = win.display_format().map_err(err_msg)?;
    Ok(image)
}

pub fn draw_bevel(s: &mut Surface, rect: Rect, raised: bool, size: u16) {
    let mut k;
    let mut f;
    let k_adv;
    let f_adv;
    if raised {
        k = 2.6;
        f = 0.1;
        k_adv = -0.2;
        f_adv = 0.1;
    } else {
        k = 0.1;
        f = 2.6;
        k_adv = 0.1;
        f_adv = -0.2;
    }
    let left = rect.left() as u16;
    let top = rect.top() as u16;
    let width = rect.width() as u16;
    let height = rect.height() as u16;
    for i in 0..size {
        for j in i..(height - i) {
            adjust_brightness_pixel(s, left + i, top + j, k);
        }
        for j in i..(width - i) {
            adjust_brightness_pixel(s, left + j, top + i, k);
        }
        for j in (i + 1)..(height - i) {
            adjust_brightness_pixel(s, left + width - i - 1, top + j, f);
        }
        for j in i..(width - i) {
            adjust_brightness_pixel(s, left + j, top + height - i - 1, f);
        }
        k += k_adv;
        f += f_adv;
    }
}

#[derive(Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right
}

#[derive(Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom
}

pub fn draw_text(surface: &Surface,
    text: &str,
    font: &Font, color: Color, shadow: bool,
    rect: Rect, horizontal_align: HorizontalAlign, vertical_align: VerticalAlign) -> Result<()>
{
    let (w, h) = font.size_of(text)?;

    let x = match horizontal_align {
        HorizontalAlign::Left => rect.left(),
        HorizontalAlign::Center => rect.left() + (rect.width().saturating_sub(w) as i32) / 2,
        HorizontalAlign::Right => rect.left() + (rect.width().saturating_sub(w) as i32)
    };

    let y = match vertical_align {
        VerticalAlign::Top => rect.top(),
        VerticalAlign::Middle => rect.top() + (rect.height().saturating_sub(h) as i32) / 2,
        VerticalAlign::Bottom => rect.top() + (rect.height().saturating_sub(h) as i32)
    };

    if shadow {
        let shadow_surface = font.render(text).blended(Color::RGBA(1, 1, 1, 1))?;
        unsafe {
            surface.blit_at(&surface2_to_surface(&shadow_surface), (x + 1) as i16, (y + 1) as i16);
        }
    }
    let text_surface = font.render(text).blended(color.clone())?;
    unsafe {
        surface.blit_at(&surface2_to_surface(&text_surface), x as i16, y as i16);
    }

    Ok(())
}
