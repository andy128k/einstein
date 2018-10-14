use sdl2::pixels::Color;
use error::*;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::label::*;
use ui::widget::any_key::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::context::{Rect, HorizontalAlign};
use resources::fonts::*;

pub enum MessageType {
    Neutral,
    Success,
    Failure
}

pub fn create_message_dialog(message_type: MessageType, message: &str) -> Result<Modal<()>> {
    let screen_rect = Rect::new(0, 0, 800, 600);

    let font = text_font()?;
    let (text_width, text_height) = font.size_of(message)?;
    let width = text_width + 100;
    let height = text_height + 100;

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

    let container = Modal::<()>::new(screen_rect)
        .add(WidgetMapAction::no_action(
            Window::new(rect, bg)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new0(width, height), message, color, HorizontalAlign::Center)
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
