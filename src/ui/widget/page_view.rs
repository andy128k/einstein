use std::rc::Rc;
use debug_cell::RefCell;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
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

impl Widget<Nothing> for PageView {
    fn draw(&self, context: &Context) -> Result<()> {
        let c = context.relative(self.rect);
        for item in (*self.page.borrow()).iter() {
            match *item {
                PageItem::Text(ref text, x, y, w, h) => {
                    c.relative(Rect::new(x as i32, y as i32, w as u32, h as u32))
                        .text(text, text_font()?, Color::RGB(255, 255, 255), true, HorizontalAlign::Left, VerticalAlign::Middle)?;
                },
                PageItem::Image(ref image, x, y, _w, _h) => {
                    c.image(image, x as i32, y as i32);
                }
            }
        }
        Ok(())
    }
}
