use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use ui::context::Rect;
use ui::widget::button::*;
use ui::widget::common::*;
use ui::widget::brick::*;
use resources::manager::{ResourceManager, Resource};

const BUTTON_BG_BYTES: Resource = resource!("./btn.bmp");
const BUTTON_BG_HIGHLIGHTED_BYTES: Resource = resource!("./btn-highlighted.bmp");

pub struct GameButton {
    rect: Rect,
    text: String,
}

impl ButtonRenderer for GameButton {
    fn draw(&self, _resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick {
        let image = if highlighted {
            Background::Pattern(&BUTTON_BG_HIGHLIGHTED_BYTES, false)
        } else {
            Background::Pattern(&BUTTON_BG_BYTES, false)
        };
        Brick::new(self.rect)
            .background(image)
            .text(Text::new(&self.text).font_size(FontSize::BUTTON).color(Color::RGB(255, 255, 0)).shadow())
    }
}

pub fn new_game_button<A>(rect: Rect, text: &str, key: Option<Keycode>, action: A) -> Button<GameButton, A> {
    Button::<GameButton, A>::new(
        rect,
        key,
        action,
        GameButton {
            rect,
            text: text.to_string(),
        }
    )
}
