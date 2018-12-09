use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::label::*;
use crate::ui::widget::any_key::*;
use crate::ui::widget::container::Container;
use crate::ui::context::{Rect, HorizontalAlign};

pub enum MessageType {
    Neutral,
    Success,
    Failure
}

pub fn create_message_dialog(message_type: MessageType, message: &str) -> Result<Container<()>> {
    let width = 500;
    let height = 400;

    let rect = Rect::new(
        ((800 - width) / 2) as i32,
        ((600 - height) / 2) as i32,
        width,
        height
    );

    let (bg, color) = match message_type {
        MessageType::Neutral => (Background::WHITE_PATTERN, Color::RGB(255, 0, 0)),
        MessageType::Success => (Background::GREEN_PATTERN, Color::RGB(255, 255, 0)),
        MessageType::Failure => (Background::RED_PATTERN, Color::RGB(255, 255, 255))
    };

    let container = Container::<()>::modal(rect, bg)
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new0(width, height), message, color, HorizontalAlign::Center)
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
