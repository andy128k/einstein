use std::cell::Cell;
use sdl::event::{Mouse};
use error::*;
use ui::context::{Context, Rect};
use ui::widget::common::BackgroundPattern;
use ui::widget::widget::*;
use resources::manager::ResourceManager;

pub struct Slider {
    rect: Rect,
    background: BackgroundPattern,
    value: Cell<f32>,
    highlight: Cell<bool>,
    dragging: Cell<Option<i32>>,
}

impl Slider {
    pub fn new(rect: Rect, background: BackgroundPattern, value: f32) -> Self {
        Self {
            rect,
            background,
            value: Cell::new(value),
            highlight: Cell::new(false),
            dragging: Cell::new(None),
        }
    }

    fn value_to_x(&self, value: f32) -> i32 {
        let width = self.rect.width();
        let slider_width = self.rect.height();
        let x_range = width - slider_width;
        (value.max(0f32).min(1f32) * (x_range as f32)) as i32
    }

    fn x_to_value(&self, pos: i32) -> f32 {
        let width = self.rect.width();
        let slider_width = self.rect.height();
        let x_range = width - slider_width;
        (pos.max(0).min(x_range as i32) as f32) / (x_range as f32)
    }

    fn get_slider_rect(&self) -> Rect {
        let rect = self.get_client_rect();
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
            let inside = self.get_client_rect().contains_point(p);
            if inside {
                let slider_x = self.value_to_x(self.value.get());
                let slider_height = self.get_client_rect().height();
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
    fn is_relative(&self) -> bool { true }
    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&mut self, event: &Event) -> EventResult<f32> {
        match *event {
            Event::MouseButtonDown(Mouse::Left, x, y) if self.get_slider_rect().contains_point((x, y)) => {
                self.drag_start(x, y);
                Ok(EventReaction::empty())
            },
            Event::MouseButtonUp(..) => {
                self.drag_stop();
                Ok(EventReaction::update_and_action(self.get_rect(), self.value.get()))
            },
            Event::MouseMove(x, y) => {
                if self.on_mouse_move(x, y) {
                    Ok(EventReaction::update(self.get_rect()))
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager) -> Result<()> {
        let rect = self.rect;

        context.relative(Rect::new(0, rect.height() as i32 / 2 - 2, rect.width(), 4))
            .bevel(false, 1);

        let x = self.value_to_x(self.value.get());
        let slider_rect = Rect::new(x as i32, 0, rect.height(), rect.height());
        let slider_context = context.relative(slider_rect);
        let bg = if self.highlight.get() {
            self.background.load_highlighted(resource_manager)
        } else {
            self.background.load(resource_manager)
        };
        slider_context.tiles(bg);
        slider_context.etched();
        Ok(())
    }
}
