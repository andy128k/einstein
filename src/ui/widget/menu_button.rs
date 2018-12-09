use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::context::Rect;
use crate::ui::widget::button::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;

pub struct MenuButton {
    rect: Rect,
    text: String,
}

impl ButtonRenderer for MenuButton {
    fn draw(&self, _resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick {
        let color = if highlighted {
            Color::RGB(150, 255, 255)
        } else {
            Color::RGB(30, 255, 255)
        };
        Brick::new(self.rect.width(), self.rect.height())
            .text(Text::new(&self.text).font_size(FontSize(20)).color(color).shadow())
    }
}

pub fn new_menu_button<A>(rect: Rect, text: &str, key: Option<Keycode>, action: A) -> Button<MenuButton, A> {
    Button::<MenuButton, A>::new(
        rect,
        key,
        action,
        MenuButton {
            rect,
            text: text.to_string(),
        }
    )
}
