use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::label::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::any_key::*;
use crate::ui::component::dialog::*;
use crate::resources::messages::Messages;

pub fn new_pause_dialog(messages: &Messages) -> Container<()> {
    let size = Size::new(240, 50);

    let container = dialog_container(size, DialogTheme::Green)
        .add(0, 0,
            Label::new(size, messages.paused, Color::RGB(255, 255, 0), HorizontalAlign::Center).no_action()
        )
        .add(0, 0,
            AnyKey::new(())
        );

    dialog_widget(None, container)
}
