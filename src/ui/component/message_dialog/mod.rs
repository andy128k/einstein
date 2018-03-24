use std::ffi::CStr;
use sdl;
use sdl::video::{Surface};
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::any_key::*;
use ui::widget::window::*;
use ui::widget::container::*;
use ui::utils::{HorizontalAlign, VerticalAlign, rect_to_rect2};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::background::{MARBLE_PATTERN, GREEN_PATTERN, RED_PATTERN};
use locale::get_language;

pub enum MessageType {
    Neutral,
    Success,
    Failure
}

fn create_message(screen_rect: Rect, message_type: MessageType, message: &str) -> Result<Container<()>> {
    let font = text_font()?;
    let (text_width, text_height) = font.size_of(message)?;

    let mut rect = Rect::new(0, 0, text_width + 20, text_height + 20);
    rect.center_on(screen_rect.center());

    let (bg, color) = match message_type {
        MessageType::Neutral => (MARBLE_PATTERN, Color::RGB(255, 0, 0)),
        MessageType::Success => (GREEN_PATTERN, Color::RGB(255, 255, 0)),
        MessageType::Failure => (RED_PATTERN, Color::RGB(255, 255, 255))
    };

    let mut container = Container::new(rect, ());
    container.add(Box::new(Window::new(rect, bg)?));
    container.add(Box::new(Label {
        font,
        text: message.to_string(),
        rect,
        color,
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
        shadow: true
    }));
    container.add(Box::new(AnyKey::new(|| Some(Effect::Terminate))));

    Ok(container)
}

pub fn show_message(surface: &Surface, message_type: MessageType, message: &str) -> Result<bool> {
    let message_box = create_message(rect_to_rect2(surface.get_rect()), message_type, message)?;
    main_loop(surface, &message_box)
}
