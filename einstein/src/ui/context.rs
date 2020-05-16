use crate::audio::Audio;
use crate::cell::RefCell;
use crate::error::format_err;
use crate::error::*;
use crate::resources::manager::ResourceManager;
use crate::ui::widget::widget::{Event as WidgetEvent, EventReaction, Widget};
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct MainLoopQuit;

pub trait Context {
    fn resource_manager(&self) -> &dyn ResourceManager;
    fn audio(&self) -> &dyn Audio;
    fn main_loop(&self, widget: &mut dyn Widget<MainLoopQuit>) -> Result<()>;
}

pub struct AppContext<'c> {
    pub sdl_context: &'c Sdl,
    pub canvas: RefCell<&'c mut Canvas<Window>>,
    pub resource_manager: &'c dyn ResourceManager,
    pub audio: &'c dyn Audio,
}

impl Context for AppContext<'_> {
    fn resource_manager(&self) -> &dyn ResourceManager {
        self.resource_manager
    }
    fn audio(&self) -> &dyn Audio {
        self.audio
    }

    fn main_loop(&self, widget: &mut dyn Widget<MainLoopQuit>) -> Result<()> {
        self.canvas.borrow_mut().clear();

        let b = widget.draw(self.resource_manager);
        b.draw(&mut *self.canvas.borrow_mut(), 0, 0, self.resource_manager)?;
        self.canvas.borrow_mut().present();

        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(|e| format_err!("{}", e))?;
        loop {
            sleep(Duration::from_millis(5));
            widget.on_event(&WidgetEvent::Tick, self)?; // TODO: add timer

            for event in event_pump.poll_iter() {
                let reaction = match event {
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => widget.on_event(&WidgetEvent::KeyDown(key), self)?,
                    Event::TextInput { text, .. } => {
                        widget.on_event(&WidgetEvent::TextInput(text), self)?
                    }
                    Event::MouseMotion { x, y, .. } => {
                        widget.on_event(&WidgetEvent::MouseMove(x as i32, y as i32), self)?
                    }
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => widget.on_event(&WidgetEvent::MouseButtonDown(mouse_btn, x, y), self)?,
                    Event::MouseButtonUp {
                        mouse_btn, x, y, ..
                    } => widget.on_event(&WidgetEvent::MouseButtonUp(mouse_btn, x, y), self)?,
                    Event::Quit { .. } => return Ok(()),
                    _ => EventReaction::empty(),
                };
                if reaction.update {
                    let b = widget.draw(self.resource_manager);
                    b.draw(&mut *self.canvas.borrow_mut(), 0, 0, self.resource_manager)?;
                    self.canvas.borrow_mut().present();
                }
                if reaction.action.is_some() {
                    return Ok(());
                }
            }
        }
    }
}
