use crate::resources::messages::Messages;
use crate::ui::common::{HorizontalAlign, Size};
use crate::ui::component::dialog::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::*;
use crate::ui::widget::widget::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn create_about_dialog(messages: &Messages) -> Container<()> {
    let theme = DialogTheme::Blue;

    let container = dialog_container(Size::new(360, 280), theme)
        .add(
            30,
            5,
            Label::title(Size::new(300, 40), messages.about).no_action(),
        )
        .add(
            0,
            80,
            Label::new(
                Size::new(360, 20),
                messages.einstein_puzzle,
                Color::RGB(255, 255, 255),
                HorizontalAlign::Center,
            )
            .no_action(),
        )
        .add(
            0,
            100,
            Label::new(
                Size::new(360, 20),
                &format!("{}{}", messages.version, VERSION),
                Color::RGB(255, 255, 255),
                HorizontalAlign::Center,
            )
            .no_action(),
        )
        .add(
            0,
            120,
            Label::new(
                Size::new(360, 20),
                messages.copyright,
                Color::RGB(255, 255, 255),
                HorizontalAlign::Center,
            )
            .no_action(),
        )
        .add(
            0,
            170,
            Label::new(
                Size::new(360, 20),
                "http://games.flowix.com",
                Color::RGB(255, 255, 0),
                HorizontalAlign::Center,
            )
            .no_action(),
        )
        .add(
            140,
            240,
            DialogButton::new(
                Size::new(80, 25),
                theme,
                messages.ok,
                &[Keycode::Escape, Keycode::Return],
                (),
            ),
        );

    dialog_widget(None, container)
}
