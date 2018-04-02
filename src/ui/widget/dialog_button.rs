use sdl::video::Surface;
use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::button::*;
use ui::utils::{load_image, adjust_brightness};
use resources::fonts::*;

pub struct DialogButton {
    text: String,
    image: Surface,
    highlighted_image: Surface,
}

impl ButtonRenderer for DialogButton {
    fn draw(&self, context: &Context, highlighted: bool) -> Result<()> {
        let image = if highlighted {
            &self.highlighted_image
        } else {
            &self.image
        };
        context.tiles(image);
        context.etched();
        context.text(&self.text, button_font()?, Color::RGB(255, 255, 0), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}

pub fn new_dialog_button2<A>(rect: Rect, image: &[u8], text: &str, key: Option<Key>, action: A) -> Result<Button<DialogButton, A>> {
    let bg = load_image(image)?;
    let highlighted_bg = adjust_brightness(&bg, 1.5);
    Ok(Button::<DialogButton, A>::new(
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
