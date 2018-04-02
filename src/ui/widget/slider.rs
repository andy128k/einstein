use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Mouse};
use error::*;
use ui::context::{Context, Rect};
use ui::widget::widget::*;
use ui::utils::{load_image, adjust_brightness};

pub struct Slider {
    rect: Rect,
    background: Surface,
    background_highlighted: Surface,
    value: Cell<f32>,
    highlight: Cell<bool>,
    dragging: Cell<Option<i32>>,
}

impl Slider {
    pub fn new(rect: Rect, bg: &[u8], value: f32) -> Result<Self> {
        let background = load_image(bg)?;
        let background_highlighted = adjust_brightness(&background, 1.5);

        Ok(Self {
            rect,
            background,
            background_highlighted,
            value: Cell::new(value),
            highlight: Cell::new(false),
            dragging: Cell::new(None),
        })
    }

    fn value_to_x(&self, value: f32) -> i16 {
        let width = self.rect.width();
        let slider_width = self.rect.height();
        let x_range = width - slider_width;
        (value.max(0f32).min(1f32) * (x_range as f32)) as i16
    }

    fn x_to_value(&self, pos: i16) -> f32 {
        let width = self.rect.width();
        let slider_width = self.rect.height();
        let x_range = width - slider_width;
        (pos.max(0).min(x_range as i16) as f32) / (x_range as f32)
    }

    fn get_slider_rect(&self) -> Rect {
        let slider_x = self.value_to_x(self.value.get());
        Rect::new(self.rect.left() + slider_x as i32, self.rect.top(), self.rect.height(), self.rect.height())
    }

    fn drag_start(&self, x: i32, _y: i32) {
        let slider_x = self.value_to_x(self.value.get());
        self.dragging.set(Some(x - self.rect.left() - (slider_x as i32)));
    }

    fn drag_stop(&self) {
        self.dragging.set(None);
    }

    fn on_mouse_move(&self, x: i32, y: i32) -> EventReaction<f32> {
        if let Some(drag_x) = self.dragging.get() {
            let val = self.x_to_value((x - self.rect.left() - drag_x) as i16);
            if val != self.value.get() {
                self.value.set(val);
                EventReaction::Redraw
            } else {
                EventReaction::NoOp
            }
        } else {
            let p = (x, y);
            let inside = self.rect.contains_point(p);
            if inside {
                let slider_x = self.value_to_x(self.value.get());
                let slider_rect = Rect::new(self.rect.left() + slider_x as i32, self.rect.top(), self.rect.height(), self.rect.height());
                let highlight = slider_rect.contains_point(p);
                if highlight != self.highlight.get() {
                    self.highlight.set(highlight);
                }
                EventReaction::Redraw
            } else {
                if self.highlight.get() {
                    self.highlight.set(false);
                    EventReaction::Redraw
                } else {
                    EventReaction::NoOp
                }
            }
        }
    }
}

impl Widget<f32> for Slider {
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&self, event: &Event) -> EventReaction<f32> {
        match *event {
            Event::MouseButtonDown(Mouse::Left, x, y) if self.get_slider_rect().contains_point((x, y)) => {
                self.drag_start(x, y);
                EventReaction::NoOp
            },
            Event::MouseButtonUp(..) => {
                self.drag_stop();
                EventReaction::Action(self.value.get())
            },
            Event::MouseMove(x, y) => self.on_mouse_move(x, y),
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        let rect = self.rect;
        let c = context.relative(self.rect);

        c.relative(Rect::new(0, rect.height() as i32 / 2 - 2, rect.width(), 4))
            .bevel(false, 1);

        let x = self.value_to_x(self.value.get());
        let slider_rect = Rect::new(x as i32, 0, rect.height(), rect.height());
        let slider_context = c.relative(slider_rect);
        if self.highlight.get() {
            slider_context.tiles(&self.background_highlighted);
        } else {
            slider_context.tiles(&self.background);
        }
        slider_context.etched();
        Ok(())
    }
}
