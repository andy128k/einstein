use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::{Widget, Nothing};
use ui::context::{Context, HorizontalAlign, VerticalAlign};
use resources::fonts::title_font;

pub struct Title {
    pub text: String,
    pub rect: Rect,
}

impl Widget<Nothing> for Title {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context) -> Result<()> {
        context.text(&self.text, title_font()?, Color::RGB(255, 255, 0), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}
