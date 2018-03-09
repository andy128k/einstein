use std::thread::sleep;
use std::time::Duration;
use sdl;
use sdl::video::Surface;
use sdl::event::{Event, poll_event};
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;

fn rect2_to_rect(rect: &Rect) -> sdl::sdl::Rect {
    sdl::sdl::Rect::new(rect.left() as i16, rect.top() as i16, rect.width() as u16, rect.height() as u16)
}

pub fn main_loop(surface: &Surface, widget: &Widget) -> Result<bool> {
    widget.draw(surface)?;
    surface.update_rects(&[rect2_to_rect(&widget.get_rect())]);

    loop {
        sleep(Duration::from_millis(5));
        let event = poll_event();
        let effect = match event {
            Event::None => widget.on_tick(),
            Event::Key(key, _, _, ch) => widget.on_key_down(key, ch),
            Event::MouseMotion(_, x, y, _, _) => widget.on_mouse_move(x, y),
            Event::MouseButton(mouse, true, x, y) => widget.on_mouse_button_down(mouse, x, y),
            Event::MouseButton(mouse, false, x, y) => widget.on_mouse_button_up(mouse, x, y),
            Event::Quit => return Ok(true),
            _ => None
        };
        match effect {
            Some(Effect::Terminate) => return Ok(false),
            Some(Effect::Redraw(rects)) => {
                widget.draw(surface)?;
                let rects1 = rects.iter().map(rect2_to_rect).collect::<Vec<_>>();
                surface.update_rects(&rects1);
            },
            Some(Effect::NoOp) |
            None => {}
        }
    }
}
