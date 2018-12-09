use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::context::{Rect, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::label::*;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::container::Container;
use crate::resources::messages::Messages;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn create_about_dialog(messages: &Messages) -> Result<Container<()>> {
    let rect = Rect::new(220, 160, 360, 280);
    let bg = Background::BLUE_PATTERN;
    let container = Container::<()>::modal(rect, bg)
        .add(WidgetMapAction::no_action(
            Label::title(Rect::new(30, 5, 300, 40), messages.about)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 80, 360, 20), messages.einstein_puzzle, Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 100, 360, 20), &format!("{}{}", messages.version, VERSION), Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 120, 360, 20), messages.copyright, Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 170, 360, 20),"http://games.flowix.com", Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(
            DialogButton::new(Rect::new(140, 240, 80, 25), bg, messages.ok,
                Some(Keycode::Escape), // Return also
                ()
            )
        );
    Ok(container)
}
