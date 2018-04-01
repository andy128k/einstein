use sdl::video::{Surface};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::any_key::*;
use ui::widget::window::*;
use ui::widget::dialog::*;
use ui::utils::{HorizontalAlign, VerticalAlign, rect_to_rect2};
use ui::main_loop::{main_loop, ModalResult};
use resources::fonts::*;
use resources::background::{MARBLE_PATTERN, GREEN_PATTERN, RED_PATTERN};

pub enum MessageType {
    Neutral,
    Success,
    Failure
}

pub fn create_message_dialog(message_type: MessageType, message: &str) -> Result<WidgetPtr<ModalResult<()>>> {
    let screen_rect = Rect::new(0, 0, 800, 600);

    let font = text_font()?;
    let (text_width, text_height) = font.size_of(message)?;

    let mut rect = Rect::new(0, 0, text_width + 100, text_height + 100);
    rect.center_on(screen_rect.center());

    let (bg, color) = match message_type {
        MessageType::Neutral => (MARBLE_PATTERN, Color::RGB(255, 0, 0)),
        MessageType::Success => (GREEN_PATTERN, Color::RGB(255, 255, 0)),
        MessageType::Failure => (RED_PATTERN, Color::RGB(255, 255, 255))
    };

    let container: Vec<WidgetPtr<ModalResult<()>>> = vec![
        Box::new(
            InterceptWidget::default()
        ),
        Box::new(WidgetMapAction::no_action(
            Window::new(rect, bg)?
        )),
        Box::new(WidgetMapAction::no_action(
            Label {
                text: message.to_string(),
                rect,
                color,
                horizontal_align: HorizontalAlign::Center,
                vertical_align: VerticalAlign::Middle,
            }
        )),
        Box::new(AnyKey::new(ModalResult(()))),
    ];

    Ok(Box::new(container))
}
