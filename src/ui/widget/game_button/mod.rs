use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::button::*;
use resources::manager::ResourceManager;
use resources::fonts::*;

const BUTTON_BG_BYTES: &[u8] = include_bytes!("./btn.bmp");
const BUTTON_BG_HIGHLIGHTED_BYTES: &[u8] = include_bytes!("./btn-highlighted.bmp");

pub struct GameButton {
    text: String,
}

impl ButtonRenderer for GameButton {
    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager, highlighted: bool) -> Result<()> {
        let image = if highlighted {
            resource_manager.image("BUTTON_BG_HIGHLIGHTED_BYTES", BUTTON_BG_HIGHLIGHTED_BYTES)
        } else {
            resource_manager.image("BUTTON_BG_BYTES", BUTTON_BG_BYTES)
        };
        context.image(image, 0, 0);
        context.text(&self.text, button_font()?, Color::RGB(255, 255, 0), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}

pub fn new_game_button<A>(rect: Rect, text: &str, key: Option<Key>, action: A) -> Button<GameButton, A> {
    Button::<GameButton, A>::new(
        rect,
        key,
        action,
        GameButton {
            text: text.to_string(),
        }
    )
}
