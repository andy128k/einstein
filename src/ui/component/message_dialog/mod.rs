use sdl2::pixels::Color;
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::any_key::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::context::{Rect, HorizontalAlign, VerticalAlign};
use resources::fonts::*;
use resources::background::{MARBLE_PATTERN, GREEN_PATTERN, RED_PATTERN};

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
        MessageType::Neutral => (MARBLE_PATTERN, Color::RGB(255, 0, 0)),
        MessageType::Success => (GREEN_PATTERN, Color::RGB(255, 255, 0)),
        MessageType::Failure => (RED_PATTERN, Color::RGB(255, 255, 255))
    };

    let container = Modal::<()>::new(screen_rect)
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new0(width, height), bg)?
        ))
        .add(WidgetMapAction::no_action(
            Label {
                text: message.to_string(),
                rect,
                color,
                horizontal_align: HorizontalAlign::Center,
                vertical_align: VerticalAlign::Middle,
            }
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
