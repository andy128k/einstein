use std::thread::sleep;
use std::time::Duration;
use sdl::video::Surface;
use sdl::event::{Event, poll_event};
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::{Widget, Event as WidgetEvent, EventReaction};
use ui::utils::rect2_to_rect;

#[derive(Clone)]
pub struct ModalResult<T>(pub T);

pub fn main_loop<T>(surface: &Surface, rect: Rect, widget: &Widget<ModalResult<T>>) -> Result<Option<T>> {
    widget.draw(surface)?;
    surface.update_rects(&[rect2_to_rect(rect)]);

    loop {
        sleep(Duration::from_millis(5));
        let event = poll_event();
        let reaction = match event {
            Event::None => widget.on_event(&WidgetEvent::Tick),
            Event::Key(key, _, _, ch) => widget.on_event(&WidgetEvent::KeyDown(key, ch)),
            Event::MouseMotion(_, x, y, _, _) => widget.on_event(&WidgetEvent::MouseMove(x as i32, y as i32)),
            Event::MouseButton(mouse, true, x, y) => widget.on_event(&WidgetEvent::MouseButtonDown(mouse, x as i32, y as i32)),
            Event::MouseButton(mouse, false, x, y) => widget.on_event(&WidgetEvent::MouseButtonUp(mouse, x as i32, y as i32)),
            Event::Quit => return Ok(None),
            _ => EventReaction::NoOp
        };
        match reaction {
            EventReaction::Action(ModalResult(value)) => {
                widget.draw(surface)?;
                surface.update_rects(&[rect2_to_rect(rect)]);
                return Ok(Some(value));
            },
            EventReaction::Redraw => {
                widget.draw(surface)?;
                surface.update_rects(&[rect2_to_rect(rect)]);
            },
            EventReaction::StopPropagation |
            EventReaction::NoOp => {},
        }
    }
}
