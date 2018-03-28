use sdl::video::Surface;
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::button::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;

pub struct MenuButton {
    text: String,
}

impl ButtonRenderer for MenuButton {
    fn draw(&self, surface: &Surface, rect: Rect, highlighted: bool) -> Result<()> {
        let color = if highlighted {
            Color::RGB(150, 255, 255)
        } else {
            Color::RGB(30, 255, 255)
        };
        draw_text(surface, &self.text, menu_font()?, color, true, rect, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}

pub fn new_menu_button<A: Fn() -> Option<Effect> + 'static>(rect: Rect, text: &str, key: Option<Key>, action: A) -> Button<MenuButton> {
    Button::<MenuButton>::new(
        rect,
        key,
        action,
        MenuButton {
            text: text.to_string(),
        }
    )
}
