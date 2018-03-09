use std::rc::{Rc, Weak};
use std::cell::RefCell;
use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;

pub struct Container<T> {
    rect: Rect,
    widgets: Vec<Box<Widget>>,
    private: Rc<RefCell<T>>
}

impl<T> Container<T> {
    pub fn new(rect: Rect, private: T) -> Self {
        Self {
            rect,
            widgets: Vec::new(),
            private: Rc::new(RefCell::new(private))
        }
    }

    pub fn add(&mut self, widget: Box<Widget>) {
        self.widgets.push(widget);
    }

    pub fn weak_private(&self) -> Weak<RefCell<T>> {
        Rc::downgrade(&self.private)
    }
}

impl<T> Widget for Container<T> {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_tick(&self) -> Option<Effect> {
        for widget in &self.widgets {
            let effect = widget.on_tick();
            if effect.is_some() {
                return effect;
            }
        }
        None
    }

    fn on_mouse_button_down(&self, button: Mouse, x: u16, y: u16) -> Option<Effect> {
        for widget in &self.widgets {
            let effect = widget.on_mouse_button_down(button, x, y);
            if effect.is_some() {
                return effect;
            }
        }
        None
    }

    fn on_mouse_button_up(&self, button: Mouse, x: u16, y: u16) -> Option<Effect> {
        for widget in &self.widgets {
            let effect = widget.on_mouse_button_up(button, x, y);
            if effect.is_some() {
                return effect;
            }
        }
        None
    }

    fn on_mouse_move(&self, x: u16, y: u16) -> Option<Effect> {
        for widget in &self.widgets {
            let effect = widget.on_mouse_move(x, y);
            if effect.is_some() {
                return effect;
            }
        }
        None
    }

    fn on_key_down(&self, key: Key, ch: u16) -> Option<Effect> {
        for widget in &self.widgets {
            let effect = widget.on_key_down(key, ch);
            if effect.is_some() {
                return effect;
            }
        }
        None

    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        for widget in &self.widgets {
            widget.draw(surface)?;
        }
        Ok(())
    }
}
