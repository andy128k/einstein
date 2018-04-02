use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::button::*;
use resources::fonts::*;

pub struct MenuButton {
    text: String,
}

impl ButtonRenderer for MenuButton {
    fn draw(&self, context: &Context, highlighted: bool) -> Result<()> {
        let color = if highlighted {
            Color::RGB(150, 255, 255)
        } else {
            Color::RGB(30, 255, 255)
        };
        context.text(&self.text, menu_font()?, color, true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}

pub fn new_menu_button<A>(rect: Rect, text: &str, key: Option<Key>, action: A) -> Button<MenuButton, A> {
    Button::<MenuButton, A>::new(
        rect,
        key,
        action,
        MenuButton {
            text: text.to_string(),
        }
    )
}
