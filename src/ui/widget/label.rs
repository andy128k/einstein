use sdl::video::Surface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::context::Context;
use ui::widget::widget::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::text_font;

pub struct Label {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
}

impl Widget<Nothing> for Label {
    fn draw(&self, context: &Context) -> Result<()> {
        let c = context.relative(self.rect);
        c.text(&self.text, text_font()?, self.color, true, self.horizontal_align, self.vertical_align)?;
        Ok(())
    }
}
