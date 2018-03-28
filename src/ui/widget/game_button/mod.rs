use sdl::video::Surface;
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::button::*;
use ui::utils::{load_image, adjust_brightness, draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;

const BUTTON_BG_BYTES: &[u8] = include_bytes!("./btn.bmp");

struct Resources {
    game_button_bg: Surface,
    game_button_bg_highlighted: Surface,
}

thread_local!(static RESOURCES: Resources = init_resources());

fn init_resources() -> Resources {
    let game_button_bg = load_image(BUTTON_BG_BYTES).unwrap();
    let game_button_bg_highlighted = adjust_brightness(&game_button_bg, 1.5);
    Resources {
        game_button_bg,
        game_button_bg_highlighted,
    }
}

pub struct GameButton {
    text: String,
}

impl ButtonRenderer for GameButton {
    fn draw(&self, surface: &Surface, rect: Rect, highlighted: bool) -> Result<()> {
        RESOURCES.with(|res| {
            let image = if highlighted {
                &res.game_button_bg_highlighted
            } else {
                &res.game_button_bg
            };
            surface.blit_at(image, rect.left() as i16, rect.top() as i16);
            draw_text(surface, &self.text, button_font()?, Color::RGB(255, 255, 0), true, rect, HorizontalAlign::Center, VerticalAlign::Middle)?;
            Ok(())
        })
    }
}

pub fn new_game_button<A: Fn() -> Option<Effect> + 'static>(rect: Rect, text: &str, key: Option<Key>, action: A) -> Button<GameButton> {
    Button::<GameButton>::new(
        rect,
        key,
        action,
        GameButton {
            text: text.to_string(),
        }
    )
}
