use std::cell::Cell;
use failure::err_msg;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use crate::ui::common::Size;
use crate::ui::widget::common::*;
use crate::ui::widget::widget::*;
use crate::ui::brick::*;
use crate::ui::context::Context;
use crate::resources::manager::ResourceManager;
use crate::resources::audio::CLICK;

pub struct Checkbox {
    background: Background,
    checked: Cell<bool>,
    mouse_inside: Cell<bool>,
}

impl Checkbox {
    pub fn new(background: Background, checked: bool) -> Self {
        Self{
            background,
            checked: Cell::new(checked),
            mouse_inside: Cell::new(false),
        }
    }
}

impl Widget<bool> for Checkbox {
    fn get_size(&self) -> Size {
        Size { width: 20, height: 20 }
    }

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<bool> {
        let rect = self.get_size().to_rect();
        match *event {
            Event::MouseButtonDown(MouseButton::Left, x, y) if rect.contains_point((x, y)) => {
                context.audio().play(&*context.resource_manager().chunk(&CLICK)).map_err(err_msg)?;
                self.checked.set(!self.checked.get());
                Ok(EventReaction::update_and_action(self.checked.get()))
            },
            Event::MouseMove(x, y) => {
                let to_highlight = rect.contains_point((x, y));
                if self.mouse_inside.get() != to_highlight {
                    self.mouse_inside.set(to_highlight);
                    Ok(EventReaction::update())
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_size().width, self.get_size().height)
            .background(if self.mouse_inside.get() { self.background.highlighted() } else { self.background })
            .border(Border::Etched);
        if self.checked.get() {
            brick = brick.text(Text::new("X").color(Color::RGB(255, 255, 255)).shadow());
        }
        brick
    }
}
