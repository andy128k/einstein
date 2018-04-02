use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use resources::fonts::text_font;

pub struct Label {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
}

impl Widget<Nothing> for Label {
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context) -> Result<()> {
        let c = context.relative(self.rect);
        c.text(&self.text, text_font()?, self.color, true, self.horizontal_align, self.vertical_align)?;
        Ok(())
    }
}
