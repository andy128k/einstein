use sdl::event::{Key};
use sdl2::pixels::Color;
use ui::context::Rect;
use ui::widget::button::*;
use ui::widget::common::*;
use ui::widget::brick::*;
use resources::manager::ResourceManager;

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
        Brick::new(self.rect)
            .text(Text::new(&self.text).font_size(FontSize(20)).color(color).shadow())
    }
}

pub fn new_menu_button<A>(rect: Rect, text: &str, key: Option<Key>, action: A) -> Button<MenuButton, A> {
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
