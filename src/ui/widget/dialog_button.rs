use sdl::event::{Key};
use sdl2::pixels::Color;
use ui::context::Rect;
use ui::widget::brick::*;
use ui::widget::button::*;
use ui::widget::common::*;
use resources::manager::ResourceManager;

pub struct DialogButton {
    rect: Rect,
    text: String,
    background: BackgroundPattern,
}

impl ButtonRenderer for DialogButton {
    fn draw(&self, _resource_manager: &mut ResourceManager, highlighted: bool) -> Brick {
        Brick::new(self.rect)
            .background(if highlighted { self.background.highlighted() } else { self.background })
            .border(Border::Etched)
            .text(Text::new(&self.text).font_size(FontSize::BUTTON).color(Color::RGB(255, 255, 0)).shadow())
    }
}

impl DialogButton {
    pub fn new<A>(rect: Rect, background: BackgroundPattern, text: &str, key: Option<Key>, action: A) -> Button<DialogButton, A> {
        Button::<DialogButton, A>::new(
            rect,
            key,
            action,
            DialogButton {
                rect,
                text: text.to_string(),
                background,
            }
        )
    }
}
