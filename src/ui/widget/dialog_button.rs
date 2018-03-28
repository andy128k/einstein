use sdl::video::Surface;
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::button::*;
use ui::utils::{load_image, draw_bevel, draw_tiles, adjust_brightness, draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;

pub struct DialogButton {
    text: String,
    image: Surface,
    highlighted_image: Surface,
}

impl ButtonRenderer for DialogButton {
    fn draw(&self, surface: &Surface, rect: Rect, highlighted: bool) -> Result<()> {
        let image = if highlighted {
            &self.highlighted_image
        } else {
            &self.image
        };
        draw_tiles(surface, rect, image);

        let inner_rect = Rect::new(rect.left() + 1, rect.top() + 1, rect.width() - 2, rect.height() - 2);
        draw_bevel(surface, inner_rect, true, 1);
        draw_bevel(surface, rect, false, 1);

        draw_text(surface, &self.text, button_font()?, Color::RGB(255, 255, 0), true, rect, HorizontalAlign::Center, VerticalAlign::Middle)?;

        Ok(())
    }
}

pub fn new_dialog_button<A: Fn() -> Option<Effect> + 'static>(rect: Rect, image: &[u8], text: &str, key: Option<Key>, action: A) -> Result<Button<DialogButton>> {
    let bg = load_image(image)?;
    let highlighted_bg = adjust_brightness(&bg, 1.5);
    Ok(Button::<DialogButton>::new(
        rect,
        key,
        action,
        DialogButton {
            text: text.to_string(),
            image: bg,
            highlighted_image: highlighted_bg,
        }
    ))
}
