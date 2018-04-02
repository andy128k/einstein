use sdl::video::Surface;
use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::button::*;
use ui::utils::{load_image, adjust_brightness};
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
    fn draw(&self, context: &Context, highlighted: bool) -> Result<()> {
        RESOURCES.with(|res| {
            let image = if highlighted {
                &res.game_button_bg_highlighted
            } else {
                &res.game_button_bg
            };
            context.image(image, 0, 0);
            context.text(&self.text, button_font()?, Color::RGB(255, 255, 0), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
            Ok(())
        })
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
