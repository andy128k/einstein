use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use resources::manager::ResourceManager;
use resources::fonts::text_font;

pub struct Label {
    text: String,
    rect: Rect,
    color: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
}

impl Label {
    pub fn new(rect: Rect, text: &str, color: Color, horizontal_align: HorizontalAlign) -> Self {
        Self {
            text: text.to_string(),
            rect,
            color,
            horizontal_align,
            vertical_align: VerticalAlign::Middle,
        }
    }
}

impl Widget<Nothing> for Label {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager) -> Result<()> {
        context.text(&self.text, text_font()?, self.color, true, self.horizontal_align, self.vertical_align)?;
        Ok(())
    }
}
