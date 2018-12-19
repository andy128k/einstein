use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::common::Size;
use crate::ui::widget::button::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;

pub struct MenuButton {
    size: Size,
    text: String,
}

impl ButtonRenderer for MenuButton {
    fn draw(&self, _resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick {
        let color = if highlighted {
            Color::RGB(150, 255, 255)
        } else {
            Color::RGB(30, 255, 255)
        };
        Brick::new(self.size.width, self.size.height)
            .text(Text::new(&self.text).font_size(FontSize(20)).color(color).shadow())
    }
}

pub fn new_menu_button<A>(size: Size, text: &str, key: Option<Keycode>, action: A) -> Button<MenuButton, A> {
    Button::<MenuButton, A>::new(
        size,
        key,
        action,
        MenuButton {
            size,
            text: text.to_string(),
        }
    )
}
