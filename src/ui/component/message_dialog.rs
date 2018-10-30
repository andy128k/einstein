use sdl2::pixels::Color;
use error::*;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::label::*;
use ui::widget::any_key::*;
use ui::widget::container::Container;
use ui::context::{Rect, HorizontalAlign};

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
        MessageType::Neutral => (BackgroundPattern::White, Color::RGB(255, 0, 0)),
        MessageType::Success => (BackgroundPattern::Green, Color::RGB(255, 255, 0)),
        MessageType::Failure => (BackgroundPattern::Red, Color::RGB(255, 255, 255))
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
