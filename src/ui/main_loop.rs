use std::thread::sleep;
use std::time::Duration;
use sdl::event::{Event, poll_event};
use error::*;
use ui::context::{Context, rect_to_rect1};
use ui::widget::widget::{Widget, Event as WidgetEvent, EventReaction};
use resources::manager::ResourceManager;

#[derive(Clone)]
pub struct MainLoopQuit;

pub fn main_loop(context: &Context, widget: &mut Widget<MainLoopQuit>, resource_manager: &mut ResourceManager) -> Result<()> {
    widget.draw(context, resource_manager)?;
    context.surface.update_rects(&[rect_to_rect1(context.rect)]);

    loop {
        sleep(Duration::from_millis(5));
        let event = poll_event();
        let reaction = match event {
            Event::None => widget.on_event(&WidgetEvent::Tick)?,
            Event::Key(key, _, _, ch) => widget.on_event(&WidgetEvent::KeyDown(key, ch))?,
            Event::MouseMotion(_, x, y, _, _) => widget.on_event(&WidgetEvent::MouseMove(x as i32, y as i32))?,
            Event::MouseButton(mouse, true, x, y) => widget.on_event(&WidgetEvent::MouseButtonDown(mouse, x as i32, y as i32))?,
            Event::MouseButton(mouse, false, x, y) => widget.on_event(&WidgetEvent::MouseButtonUp(mouse, x as i32, y as i32))?,
            Event::Quit => return Ok(()),
            _ => EventReaction::empty()
        };
        if reaction.update.len() > 0 {
            widget.draw(context, resource_manager)?;
            // let rects = reaction.update.iter().map(|r| rect_to_rect1(*r)).collect::<Vec<_>>();
            // context.surface.update_rects(&rects);
            context.surface.update_rects(&[rect_to_rect1(context.rect)]);
        }
        if reaction.action.is_some() {
            return Ok(());
        }
    }
}
