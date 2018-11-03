use std::thread::sleep;
use std::time::Duration;
use sdl2::Sdl;
use sdl2::event::{Event};
use sdl2::render::{Canvas};
use sdl2::video::Window;
use failure::err_msg;
use error::*;
use ui::context::{Rect};
use ui::widget::widget::{Widget, Event as WidgetEvent, EventReaction};
use resources::manager::ResourceManager;
use audio::Audio;

#[derive(Clone)]
pub struct MainLoopQuit;

pub fn main_loop(sdl_context: &Sdl, canvas: &mut Canvas<Window>, widget: &mut Widget<MainLoopQuit>, resource_manager: &dyn ResourceManager, audio: &Audio) -> Result<()> {
    let rect = Rect::new(0, 0, 800, 600);

    canvas.clear();

    let b = widget.draw(resource_manager);
    b.draw(canvas, rect, resource_manager)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(err_msg)?;
    loop {
        sleep(Duration::from_millis(5));
        widget.on_event(&WidgetEvent::Tick, resource_manager, audio)?; // TODO: add timer

        for event in event_pump.poll_iter() {
            let reaction = match event {
                Event::KeyDown { keycode: Some(key), .. } => widget.on_event(&WidgetEvent::KeyDown(key), resource_manager, audio)?,
                Event::TextInput { text, .. } => widget.on_event(&WidgetEvent::TextInput(text), resource_manager, audio)?,
                Event::MouseMotion { x, y, .. } => widget.on_event(&WidgetEvent::MouseMove(x as i32, y as i32), resource_manager, audio)?,
                Event::MouseButtonDown { mouse_btn, x, y, .. } => widget.on_event(&WidgetEvent::MouseButtonDown(mouse_btn, x, y), resource_manager, audio)?,
                Event::MouseButtonUp { mouse_btn, x, y, .. } => widget.on_event(&WidgetEvent::MouseButtonUp(mouse_btn, x, y), resource_manager, audio)?,
                Event::Quit { .. } => return Ok(()),
                _ => EventReaction::empty()
            };
            if reaction.update {
                let b = widget.draw(resource_manager);
                b.draw(canvas, rect, resource_manager)?;
                canvas.present();
            }
            if reaction.action.is_some() {
                return Ok(());
            }
        }
    }
}
