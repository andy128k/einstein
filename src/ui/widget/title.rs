use sdl2::pixels::Color;
use error::*;
use ui::widget::widget::{Widget, Nothing};
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use resources::manager::ResourceManager;
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

    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager) -> Result<()> {
        context.text(&self.text, title_font()?, Color::RGB(255, 255, 0), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}
