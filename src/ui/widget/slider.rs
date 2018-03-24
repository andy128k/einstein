use std::rc::Rc;
use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use error::*;
use ui::widget::widget::*;
use ui::utils::{tiled_image, adjust_brightness, draw_bevel, draw_text, HorizontalAlign, VerticalAlign};
use ui::fonts::*;

pub struct Slider {
    rect: Rect,
    background: Surface,
    slider: Surface,
    active_slider: Surface,
    value: Rc<Cell<f32>>,
    highlight: Cell<bool>,
    dragging: Cell<Option<i32>>,
}

impl Slider {
    pub fn new(rect: Rect, bg: &[u8], value: Rc<Cell<f32>>) -> Result<Self> {
        let mut background = tiled_image(bg, rect.width() as u16, rect.height() as u16)?;

        background.lock();
        draw_bevel(&mut background, Rect::new(0, rect.height() as i32 / 2 - 2, rect.width(), 4), false, 1);
        background.unlock();

        let mut slider = tiled_image(bg, rect.height() as u16, rect.height() as u16)?;

        slider.lock();
        {
            let inner_rect = Rect::new(1, 1, rect.height() - 2, rect.height() - 2);
            draw_bevel(&mut slider, inner_rect, true, 1);

            let outer_rect = Rect::new(0, 0, rect.height(), rect.height());
            draw_bevel(&mut slider, outer_rect, false, 1);
        }
        slider.unlock();

        let active_slider = adjust_brightness(&mut slider, 1.5);

        Ok(Self {
            rect,
            background,
            slider,
            active_slider,
            value,
            highlight: Cell::new(false),
            dragging: Cell::new(None),
        })
    }

    fn value_to_x(&self, value: f32) -> i16 {
        let rect = self.get_rect();
        let width = rect.width();
        let slider_width = rect.height();
        let x_range = width - slider_width;
        (value.max(0f32).min(1f32) * (x_range as f32)) as i16
    }

    fn x_to_value(&self, pos: i16) -> f32 {
        let rect = self.get_rect();
        let width = rect.width();
        let slider_width = rect.height();
        let x_range = width - slider_width;
        (pos.max(0).min(x_range as i16) as f32) / (x_range as f32)
    }
}

impl Widget for Slider {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_mouse_button_down(&self, button: Mouse, x: u16, y: u16) -> Option<Effect> {
        let rect = self.get_rect();
        let p = (x as i32, y as i32);
        let inside = rect.contains_point(p);
        if inside {
            let slider_x = self.value_to_x(self.value.get());
            let slider_rect = Rect::new(rect.left() + slider_x as i32, rect.top(), rect.height(), rect.height());
            if slider_rect.contains_point(p) {
                self.dragging.set(Some((x as i32) - rect.left() - (slider_x as i32)));
            }
        }
        None
    }

    fn on_mouse_button_up(&self, _button: Mouse, _x: u16, _y: u16) -> Option<Effect> {
        self.dragging.set(None);
        None
    }

    fn on_mouse_move(&self, x: u16, y: u16) -> Option<Effect> {
        if let Some(drag_x) = self.dragging.get() {
            let val = self.x_to_value(((x as i32) - self.get_rect().left() - drag_x) as i16);
            if val != self.value.get() {
                self.value.set(val);
                return Some(Effect::Redraw(vec![self.rect]));
            }
        } else {
            let rect = self.get_rect();
            let p = (x as i32, y as i32);
            let inside = rect.contains_point(p);
            if inside {
                let slider_x = self.value_to_x(self.value.get());
                let slider_rect = Rect::new(rect.left() + slider_x as i32, rect.top(), rect.height(), rect.height());
                let highlight = slider_rect.contains_point(p);
                if highlight != self.highlight.get() {
                    self.highlight.set(highlight);
                }
                return Some(Effect::Redraw(vec![self.rect]));
            } else {
                if self.highlight.get() {
                    self.highlight.set(false);
                    return Some(Effect::Redraw(vec![self.rect]));
                }
            }
        }
        None
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        let rect = self.get_rect();
        surface.blit_at(&self.background, rect.left() as i16, rect.top() as i16);
        let x = self.value_to_x(self.value.get());
        let slider = if self.highlight.get() {
            &self.active_slider
        } else {
            &self.slider
        };
        surface.blit_at(slider, rect.left() as i16 + x, rect.top() as i16);
        Ok(())
    }
}
