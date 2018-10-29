use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::button::*;
use ui::widget::common::BackgroundPattern;
use resources::manager::ResourceManager;
use resources::fonts::*;

pub struct DialogButton {
    text: String,
    background: BackgroundPattern,
}

impl ButtonRenderer for DialogButton {
    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager, highlighted: bool) -> Result<()> {
        let image = if highlighted {
            self.background.highlighted().load(resource_manager)
        } else {
            self.background.load(resource_manager)
        };
        context.tiles(image);
        context.etched();
        context.text(&self.text, button_font()?, Color::RGB(255, 255, 0), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        Ok(())
    }
}

impl DialogButton {
    pub fn new<A>(rect: Rect, background: BackgroundPattern, text: &str, key: Option<Key>, action: A) -> Button<DialogButton, A> {
        Button::<DialogButton, A>::new(
            rect,
            key,
            action,
            DialogButton {
                text: text.to_string(),
                background,
            }
        )
    }
}
