use sdl::video::Surface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use error::*;
use ui::widget::widget::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};
use ui::fonts::text_font;

pub struct Label {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
}

impl Widget for Label {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_text(surface, &self.text, text_font()?, self.color, true, self.rect, self.horizontal_align, self.vertical_align)?;
        Ok(())
    }
}
