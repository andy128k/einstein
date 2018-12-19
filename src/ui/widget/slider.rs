use std::cell::Cell;
use sdl2::mouse::MouseButton;
use crate::ui::common::{Rect, Size};
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::widget::widget::*;
use crate::resources::manager::ResourceManager;
use crate::audio::Audio;

pub struct Slider {
    size: Size,
    background: Background,
    value: Cell<f32>,
    highlight: Cell<bool>,
    dragging: Cell<Option<i32>>,
}

impl Slider {
    pub fn new(size: Size, background: Background, value: f32) -> Self {
        Self {
            size,
            background,
            value: Cell::new(value),
            highlight: Cell::new(false),
            dragging: Cell::new(None),
        }
    }

    fn value_to_x(&self, value: f32) -> i32 {
        let width = self.size.width;
        let slider_width = self.size.height;
        let x_range = width - slider_width;
        (value.max(0f32).min(1f32) * (x_range as f32)) as i32
    }

    fn x_to_value(&self, pos: i32) -> f32 {
        let width = self.size.width;
        let slider_width = self.size.height;
        let x_range = width - slider_width;
        (pos.max(0).min(x_range as i32) as f32) / (x_range as f32)
    }

    fn get_slider_rect(&self) -> Rect {
        let rect = self.get_size().to_rect();
        let slider_x = self.value_to_x(self.value.get());
        Rect::new(rect.left() + slider_x as i32, rect.top(), rect.height(), rect.height())
    }

    fn drag_start(&self, x: i32, _y: i32) {
        let slider_x = self.value_to_x(self.value.get());
        self.dragging.set(Some(x - (slider_x as i32)));
    }

    fn drag_stop(&self) {
        self.dragging.set(None);
    }

    fn on_mouse_move(&self, x: i32, y: i32) -> bool {
        if let Some(drag_x) = self.dragging.get() {
            let val = self.x_to_value(x - drag_x);
            if val != self.value.get() {
                self.value.set(val);
                true
            } else {
                false
            }
        } else {
            let p = (x, y);
            let inside = self.get_size().to_rect().contains_point(p);
            if inside {
                let slider_x = self.value_to_x(self.value.get());
                let slider_height = self.get_size().height;
                let slider_rect = Rect::new(slider_x as i32, 0, slider_height, slider_height);
                let highlight = slider_rect.contains_point(p);
                if highlight != self.highlight.get() {
                    self.highlight.set(highlight);
                }
                true
            } else {
                if self.highlight.get() {
                    self.highlight.set(false);
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Widget<f32> for Slider {
    fn get_size(&self) -> Size {
        self.size
    }

    fn on_event(&mut self, event: &Event, _resource_manager: &dyn ResourceManager, _audio: &dyn Audio) -> EventResult<f32> {
        match *event {
            Event::MouseButtonDown(MouseButton::Left, x, y) if self.get_slider_rect().contains_point((x, y)) => {
                self.drag_start(x, y);
                Ok(EventReaction::empty())
            },
            Event::MouseButtonUp(..) => {
                self.drag_stop();
                Ok(EventReaction::update_and_action(self.value.get()))
            },
            Event::MouseMove(x, y) => {
                if self.on_mouse_move(x, y) {
                    Ok(EventReaction::update())
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        let x = self.value_to_x(self.value.get()) as u32;

        let scale = Brick::new(self.size.width, 4)
            .border(Border::Sunken);

        let slider = Brick::new(self.size.height, self.size.height)
            .background(if self.highlight.get() { self.background.highlighted() } else { self.background })
            .border(Border::Etched);

        Brick::new(self.size.width, self.size.height)
            .add(0, (self.size.height - 4) / 2, scale)
            .add(x, 0, slider)
    }
}
