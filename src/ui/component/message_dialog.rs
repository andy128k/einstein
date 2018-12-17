use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::label::*;
use crate::ui::widget::any_key::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::dialod_widget;
use crate::ui::context::{Size, HorizontalAlign};

pub enum MessageType {
    Neutral,
    Success,
    Failure
}

pub fn create_message_dialog(message_type: MessageType, message: &str) -> Result<Container<()>> {
    let size = Size::new(500, 400);

    let (bg, color) = match message_type {
        MessageType::Neutral => (Background::WHITE_PATTERN, Color::RGB(255, 0, 0)),
        MessageType::Success => (Background::GREEN_PATTERN, Color::RGB(255, 255, 0)),
        MessageType::Failure => (Background::RED_PATTERN, Color::RGB(255, 255, 255))
    };

    let container = Container::<()>::container(size, bg, Border::Raised)
        .add(0, 0, WidgetMapAction::no_action(
            Label::new(size, message, color, HorizontalAlign::Center)
        ))
        .add(0, 0,
            AnyKey::new(())
        );

    Ok(dialod_widget(None, container))
}
