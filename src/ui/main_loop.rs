use std::thread::sleep;
use std::time::Duration;
use sdl2::event::Event;
use failure::err_msg;
use crate::error::*;
use crate::ui::widget::widget::{Widget, Event as WidgetEvent, EventReaction};
use crate::ui::context::Context;

#[derive(Clone)]
pub struct MainLoopQuit;

pub fn main_loop(context: &mut Context<'_>, widget: &mut dyn Widget<MainLoopQuit>) -> Result<()> {
    context.canvas.clear();

    let b = widget.draw(context.resource_manager);
    b.draw(context.canvas, 0, 0, context.resource_manager)?;
    context.canvas.present();

    let mut event_pump = context.sdl_context.event_pump().map_err(err_msg)?;
    loop {
        sleep(Duration::from_millis(5));
        widget.on_event(&WidgetEvent::Tick, context.resource_manager, context.audio)?; // TODO: add timer

        for event in event_pump.poll_iter() {
            let reaction = match event {
                Event::KeyDown { keycode: Some(key), .. } => widget.on_event(&WidgetEvent::KeyDown(key), context.resource_manager, context.audio)?,
                Event::TextInput { text, .. } => widget.on_event(&WidgetEvent::TextInput(text), context.resource_manager, context.audio)?,
                Event::MouseMotion { x, y, .. } => widget.on_event(&WidgetEvent::MouseMove(x as i32, y as i32), context.resource_manager, context.audio)?,
                Event::MouseButtonDown { mouse_btn, x, y, .. } => widget.on_event(&WidgetEvent::MouseButtonDown(mouse_btn, x, y), context.resource_manager, context.audio)?,
                Event::MouseButtonUp { mouse_btn, x, y, .. } => widget.on_event(&WidgetEvent::MouseButtonUp(mouse_btn, x, y), context.resource_manager, context.audio)?,
                Event::Quit { .. } => return Ok(()),
                _ => EventReaction::empty()
            };
            if reaction.update {
                let b = widget.draw(context.resource_manager);
                b.draw(context.canvas, 0, 0, context.resource_manager)?;
                context.canvas.present();
            }
            if reaction.action.is_some() {
                return Ok(());
            }
        }
    }
}
