use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::context::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::label::*;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::dialod_widget;
use crate::resources::messages::Messages;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn create_about_dialog(messages: &Messages) -> Container<()> {
    let bg = Background::BLUE_PATTERN;
    let container = Container::<()>::container(Size::new(360, 280), bg, Border::Raised)
        .add(30, 5, WidgetMapAction::no_action(
            Label::title(Size::new(300, 40), messages.about)
        ))
        .add(0, 80, WidgetMapAction::no_action(
            Label::new(Size::new(360, 20), messages.einstein_puzzle, Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(0, 100, WidgetMapAction::no_action(
            Label::new(Size::new(360, 20), &format!("{}{}", messages.version, VERSION), Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(0, 120, WidgetMapAction::no_action(
            Label::new(Size::new(360, 20), messages.copyright, Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(0, 170, WidgetMapAction::no_action(
            Label::new(Size::new(360, 20),"http://games.flowix.com", Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(140, 240,
            DialogButton::new(Size::new(80, 25), bg, messages.ok,
                Some(Keycode::Escape), // Return also
                ()
            )
        );

    dialod_widget(None, container)
}
