use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::context::Size;
use crate::ui::widget::button::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::resources::manager::{ResourceManager, Resource};

const BUTTON_BG_BYTES: Resource = resource!("./btn.bmp");
const BUTTON_BG_HIGHLIGHTED_BYTES: Resource = resource!("./btn-highlighted.bmp");
const SIZE: Size = Size::new(94, 30);

pub struct GameButton {
    text: String,
}

impl ButtonRenderer for GameButton {
    fn draw(&self, _resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick {
        let image = if highlighted {
            Background::Pattern(&BUTTON_BG_HIGHLIGHTED_BYTES, false)
        } else {
            Background::Pattern(&BUTTON_BG_BYTES, false)
        };
        Brick::new(SIZE.width, SIZE.height)
            .background(image)
            .text(Text::new(&self.text).font_size(FontSize::BUTTON).color(Color::RGB(255, 255, 0)).shadow())
    }
}

pub fn new_game_button<A>(text: &str, key: Option<Keycode>, action: A) -> Button<GameButton, A> {
    Button::<GameButton, A>::new(
        SIZE,
        key,
        action,
        GameButton {
            text: text.to_string(),
        }
    )
}
