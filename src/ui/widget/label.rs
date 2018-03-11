use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use error::*;
use ui::widget::widget::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};

pub struct Label<'font> {
    pub font: &'font Font<'font, 'font>,
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub shadow: bool,
}

impl<'font> Widget for Label<'font> {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_text(surface, &self.text, self.font, self.color, self.shadow, self.rect, self.horizontal_align, self.vertical_align)?;
        Ok(())
    }
}
