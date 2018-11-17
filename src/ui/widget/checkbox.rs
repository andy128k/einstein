use std::cell::Cell;
use failure::err_msg;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use ui::context::Rect;
use ui::widget::common::*;
use ui::widget::widget::*;
use ui::brick::*;
use resources::manager::ResourceManager;
use resources::audio::CLICK;
use audio::Audio;

pub struct Checkbox {
    rect: Rect,
    background: Background,
    checked: Cell<bool>,
    mouse_inside: Cell<bool>,
}

impl Checkbox {
    pub fn new(x: i32, y: i32, background: Background, checked: bool) -> Self {
        Self{
            rect: Rect::new(x, y, 20, 20),
            background,
            checked: Cell::new(checked),
            mouse_inside: Cell::new(false),
        }
    }
}

impl Widget<bool> for Checkbox {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<bool> {
        let rect = self.get_client_rect();
        match *event {
            Event::MouseButtonDown(MouseButton::Left, x, y) if rect.contains_point((x, y)) => {
                audio.play(&*resource_manager.chunk(&CLICK)).map_err(err_msg)?;
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
        let mut brick = Brick::new(self.get_rect())
            .background(if self.mouse_inside.get() { self.background.highlighted() } else { self.background })
            .border(Border::Etched);
        if self.checked.get() {
            brick = brick.text(Text::new("X").color(Color::RGB(255, 255, 255)).shadow());
        }
        brick
    }
}
