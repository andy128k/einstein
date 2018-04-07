use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Mouse};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::utils::{load_image, adjust_brightness};
use resources::fonts::*;

pub struct Checkbox {
    rect: Rect,
    image: Surface,
    highlighted: Surface,
    checked: Cell<bool>,
    mouse_inside: Cell<bool>,
}

impl Checkbox {
    pub fn new(x: i32, y: i32, bg: &[u8], checked: bool) -> Result<Self> {
        let image = load_image(bg)?;
        let highlighted = adjust_brightness(&image, 1.5);

        Ok(Self{
            rect: Rect::new(x, y, 20, 20),
            image,
            highlighted,
            checked: Cell::new(checked),
            mouse_inside: Cell::new(false),
        })
    }
}

impl Widget<bool> for Checkbox {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&mut self, event: &Event) -> EventResult<bool> {
        let rect = self.get_client_rect();
        match *event {
            Event::MouseButtonDown(Mouse::Left, x, y) if rect.contains_point((x, y)) => {
                // sound->play(L"click.wav"); TODO
                self.checked.set(!self.checked.get());
                Ok(EventReaction::update_and_action(self.get_rect(), self.checked.get()))
            },
            Event::MouseMove(x, y) => {
                let to_highlight = rect.contains_point((x, y));
                if self.mouse_inside.get() != to_highlight {
                    self.mouse_inside.set(to_highlight);
                    Ok(EventReaction::update(self.get_rect()))
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        let image = if self.mouse_inside.get() {
            &self.highlighted
        } else {
            &self.image
        };
        context.tiles(image);
        context.etched();
        if self.checked.get() {
            context.text("X", text_font()?, Color::RGB(255, 255, 255), true, HorizontalAlign::Center, VerticalAlign::Middle)?;
        }
        Ok(())
    }
}
