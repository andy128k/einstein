use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Mouse};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::utils::{load_image, adjust_brightness, draw_tiles, draw_etched_rect, draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;

pub struct Checkbox {
    rect: Rect,
    image: Surface,
    highlighted: Surface,
    checked: Cell<bool>,
    mouse_inside: Cell<bool>,
}

impl Checkbox {
    pub fn new(rect: Rect, bg: &[u8], checked: bool) -> Result<Self> {
        let image = load_image(bg)?;
        let highlighted = adjust_brightness(&image, 1.5);

        Ok(Self{
            rect,
            image,
            highlighted,
            checked: Cell::new(checked),
            mouse_inside: Cell::new(false),
        })
    }
}

impl Widget<bool> for Checkbox {
    fn on_event(&self, event: &Event) -> EventReaction<bool> {
        match *event {
            Event::MouseButtonDown(Mouse::Left, x, y) if self.rect.contains_point((x, y)) => {
                // sound->play(L"click.wav"); TODO
                self.checked.set(!self.checked.get());
                EventReaction::Action(self.checked.get())
            },
            Event::MouseMove(x, y) => {
                let to_highlight = self.rect.contains_point((x, y));
                if self.mouse_inside.get() != to_highlight {
                    self.mouse_inside.set(to_highlight);
                    EventReaction::Redraw
                } else {
                    EventReaction::NoOp
                }
            },
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        let image = if self.mouse_inside.get() {
            &self.highlighted
        } else {
            &self.image
        };
        draw_tiles(surface, self.rect, image);
        draw_etched_rect(surface, self.rect);
        if self.checked.get() {
            draw_text(surface, "X", text_font()?, Color::RGB(255, 255, 255), true, self.rect, HorizontalAlign::Center, VerticalAlign::Middle)?;
        }
        Ok(())
    }
}
