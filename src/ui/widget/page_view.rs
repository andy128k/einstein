use std::rc::Rc;
use debug_cell::RefCell;
use sdl::video::Surface;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use error::*;
use ui::widget::widget::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;
use ui::page_layout::*;

pub struct PageView {
    rect: Rect,
    page: Rc<RefCell<Rc<Page>>>,
}

impl PageView {
    pub fn new(rect: Rect, page: Rc<RefCell<Rc<Page>>>) -> Self {
        Self { rect, page }
    }
}

impl Widget for PageView {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        for item in (*self.page.borrow()).iter() {
            match *item {
                PageItem::Text(ref text, x, y, w, h) => {
                    let r = Rect::new((self.rect.left() + x as i32), (self.rect.top() + y as i32), w as u32, h as u32);
                    draw_text(surface, text, text_font()?, Color::RGB(255, 255, 255), true, r, HorizontalAlign::Left, VerticalAlign::Middle)?;
                },
                PageItem::Image(ref image, x, y, _w, _h) => {
                    surface.blit_at(image, (self.rect.left() + x as i32) as i16, (self.rect.top() + y as i32) as i16);
                }
            }
        }
        Ok(())
    }
}
