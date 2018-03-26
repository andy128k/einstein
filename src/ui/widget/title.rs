use sdl::video::Surface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::Widget;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};
use ui::fonts::title_font;

pub struct Title {
    pub text: String,
    pub rect: Rect,
}

impl Widget for Title {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_text(surface, &self.text, title_font()?, Color::RGB(255, 255, 0), true, self.rect, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}
