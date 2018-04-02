use std::rc::Rc;
use sdl::video::Surface;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
pub use ui::utils::{HorizontalAlign, VerticalAlign};
use error::*;

#[derive(Clone, Copy, Default)]
pub struct Rect(i32, i32, u32, u32);

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Rect(x, y, w, h)
    }

    pub fn left(&self) -> i32 { self.0 }
    pub fn top(&self) -> i32 { self.1 }
    pub fn width(&self) -> u32 { self.2 }
    pub fn height(&self) -> u32 { self.3 }

    pub fn contains_point(&self, p: (i32, i32)) -> bool {
        p.0 >= self.0 && p.0 < self.0 + self.2 as i32 &&
        p.1 >= self.1 && p.1 < self.1 + self.3 as i32
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Rect(self.0 + dx, self.1 + dy, self.2, self.3)
    }

    pub fn intersection(&self, other: &Self) -> Option<Rect> {
        let left   = i32::max(self.0, other.0);
        let right  = i32::min(self.0 + self.2 as i32, other.0 + other.2 as i32);
        let top    = i32::max(self.1, other.1);
        let bottom = i32::min(self.1 + self.3 as i32, other.1 + other.3 as i32);
        if left < right && top < bottom {
            Some(Rect(left, top, (right - left) as u32, (bottom - top) as u32))
        } else {
            None
        }
    }
}

unsafe fn surface2_to_surface(surface: &::sdl2::surface::Surface) -> Surface {
    Surface { raw: surface.raw() as * mut ::sdl::video::ll::SDL_Surface, owned: false }
}

mod cpp {
    use libc::{c_int, c_double};
    use sdl::video::ll::SDL_Surface;

    extern "C" {
        pub fn adjust_brightness_pixel(image: * mut SDL_Surface, x: c_int, y: c_int, k: c_double);
    }
}

fn adjust_brightness_pixel(image: &Surface, x: u16, y: u16, k: f64) {
    unsafe { cpp::adjust_brightness_pixel(image.raw, x as i32, y as i32, k) };
}

pub fn rect_to_rect1(rect: Rect) -> ::sdl::Rect {
    ::sdl::Rect::new(rect.0 as i16, rect.1 as i16, rect.2 as u16, rect.3 as u16)
}

pub fn rect_to_rect2(rect: Rect) -> ::sdl2::rect::Rect {
    ::sdl2::rect::Rect::new(rect.0, rect.1, rect.2, rect.3)
}

pub struct Context {
    pub(crate) surface: Rc<Surface>,
    pub(crate) rect: Rect,
}

impl Context {
    pub fn relative(&self, rect: Rect) -> Self {
        let rect = rect.offset(self.rect.left(), self.rect.top());
        let rect = rect.intersection(&self.rect).unwrap_or_default();
        Context { surface: self.surface.clone(), rect }
    }

    pub fn width(&self) -> i32 {
        self.rect.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.rect.height() as i32
    }

    pub fn fill(&self, color: Color) {
        let c = ::sdl::video::Color::RGBA(
            color.r,
            color.g,
            color.b,
            color.a
        );
        self.surface.fill_rect(Some(rect_to_rect1(self.rect)), c);
    }

    pub fn image(&self, image: &Surface, x: i32, y: i32) {
        self.surface.set_clip_rect(Some(&rect_to_rect1(self.rect)));
        self.surface.blit_at(&image, (self.rect.left() + x) as i16, (self.rect.top() + y) as i16);
        self.surface.set_clip_rect(None);
    }

    pub fn tiles(&self, tile: &Surface) {
        self.surface.set_clip_rect(Some(&rect_to_rect1(self.rect)));

        let tile_width = tile.get_width();
        let tile_height = tile.get_height();

        let cw = (self.rect.width() as u16 + tile_width - 1) / tile_width;
        let ch = (self.rect.height() as u16 + tile_height - 1) / tile_height;

        for j in 0..ch {
            for i in 0..cw {
                self.surface.blit_at(&tile, self.rect.left() as i16 + (i * tile_width) as i16, self.rect.top() as i16 + (j * tile_height) as i16);
            }
        }

        self.surface.set_clip_rect(None);
    }

    pub fn bevel(&self, raised: bool, size: u16) {
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
        let left = self.rect.left() as u16;
        let top = self.rect.top() as u16;
        let width = self.rect.width() as u16;
        let height = self.rect.height() as u16;
        for i in 0..size {
            for j in i..(height - i) {
                adjust_brightness_pixel(&self.surface, left + i, top + j, k);
            }
            for j in i..(width - i) {
                adjust_brightness_pixel(&self.surface, left + j, top + i, k);
            }
            for j in (i + 1)..(height - i) {
                adjust_brightness_pixel(&self.surface, left + width - i - 1, top + j, f);
            }
            for j in i..(width - i) {
                adjust_brightness_pixel(&self.surface, left + j, top + height - i - 1, f);
            }
            k += k_adv;
            f += f_adv;
        }
    }

    pub fn etched(&self) {
        let inner_rect = Rect::new(1, 1, self.rect.width() - 2, self.rect.height() - 2);
        self.relative(inner_rect).bevel(true, 1);
        self.bevel(false, 1);
    }

    pub fn text(&self,
        text: &str,
        font: &Font, color: Color, shadow: bool,
        horizontal_align: HorizontalAlign, vertical_align: VerticalAlign) -> Result<()>
    {
        if text.is_empty() {
            return Ok(());
        }

        self.surface.set_clip_rect(Some(&rect_to_rect1(self.rect)));

        let (w, h) = font.size_of(text)?;

        let x = match horizontal_align {
            HorizontalAlign::Left => self.rect.left(),
            HorizontalAlign::Center => self.rect.left() + (self.rect.width().saturating_sub(w) as i32) / 2,
            HorizontalAlign::Right => self.rect.left() + (self.rect.width().saturating_sub(w) as i32)
        };

        let y = match vertical_align {
            VerticalAlign::Top => self.rect.top(),
            VerticalAlign::Middle => self.rect.top() + (self.rect.height().saturating_sub(h) as i32) / 2,
            VerticalAlign::Bottom => self.rect.top() + (self.rect.height().saturating_sub(h) as i32)
        };

        if shadow {
            let shadow_surface = font.render(text).blended(Color::RGBA(1, 1, 1, 1))?;
            unsafe {
                self.surface.blit_at(&surface2_to_surface(&shadow_surface), (x + 1) as i16, (y + 1) as i16);
            }
        }
        let text_surface = font.render(text).blended(color.clone())?;
        unsafe {
            self.surface.blit_at(&surface2_to_surface(&text_surface), x as i16, y as i16);
        }

        self.surface.set_clip_rect(None);

        Ok(())
    }
}
