use crate::error::format_err;
use crate::resources::audio::CLICK;
use crate::resources::manager::ResourceManager;
use crate::ui::brick::*;
use crate::ui::common::Size;
use crate::ui::context::Context;
use crate::ui::widget::widget::*;
use sdl2::mouse::MouseButton;
use std::cell::Cell;

pub struct Checkbox {
    checked: Cell<bool>,
    mouse_inside: Cell<bool>,
    draw: Box<dyn Fn(Size, bool, bool, &dyn ResourceManager) -> Brick>,
}

impl Checkbox {
    pub fn new(
        checked: bool,
        draw: impl Fn(Size, bool, bool, &dyn ResourceManager) -> Brick + 'static,
    ) -> Self {
        Self {
            checked: Cell::new(checked),
            mouse_inside: Cell::new(false),
            draw: Box::new(draw),
        }
    }
}

impl Widget<bool> for Checkbox {
    fn get_size(&self) -> Size {
        Size {
            width: 20,
            height: 20,
        }
    }

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<bool> {
        let rect = self.get_size().to_rect();
        match *event {
            Event::MouseButtonDown(MouseButton::Left, x, y) if rect.contains_point((x, y)) => {
                context
                    .audio()
                    .play(&*context.resource_manager().chunk(&CLICK))
                    .map_err(|e| format_err!("{}", e))?;
                self.checked.set(!self.checked.get());
                Ok(EventReaction::update_and_action(self.checked.get()))
            }
            Event::MouseMove(x, y) => {
                let to_highlight = rect.contains_point((x, y));
                if self.mouse_inside.get() != to_highlight {
                    self.mouse_inside.set(to_highlight);
                    Ok(EventReaction::update())
                } else {
                    Ok(EventReaction::empty())
                }
            }
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        (self.draw)(
            self.get_size(),
            self.mouse_inside.get(),
            self.checked.get(),
            resource_manager,
        )
    }
}
