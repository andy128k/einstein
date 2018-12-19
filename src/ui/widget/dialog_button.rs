use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::common::Size;
use crate::ui::brick::*;
use crate::ui::widget::button::*;
use crate::ui::widget::common::*;
use crate::resources::manager::ResourceManager;

pub struct DialogButton {
    size: Size,
    text: String,
    background: Background,
}

impl ButtonRenderer for DialogButton {
    fn draw(&self, _resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick {
        Brick::new(self.size.width, self.size.height)
            .background(if highlighted { self.background.highlighted() } else { self.background })
            .border(Border::Etched)
            .text(Text::new(&self.text).font_size(FontSize::BUTTON).color(Color::RGB(255, 255, 0)).shadow())
    }
}

impl DialogButton {
    pub fn new<A>(size: Size, background: Background, text: &str, keys: &[Keycode], action: A) -> Button<DialogButton, A> {
        Button::<DialogButton, A>::new(
            size,
            keys,
            action,
            DialogButton {
                size,
                text: text.to_string(),
                background,
            }
        )
    }
}
