use std::rc::Rc;
use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Mouse};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use error::*;
use ui::widget::widget::*;
use ui::utils::{load_image, adjust_brightness, draw_tiles, draw_etched_rect, draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;

pub struct Checkbox {
    rect: Rect,
    image: Surface,
    highlighted: Surface,
    checked: Rc<Cell<bool>>,
    mouse_inside: Cell<bool>,
}

impl Checkbox {
    pub fn new(rect: Rect, bg: &[u8], checked: Rc<Cell<bool>>) -> Result<Self> {
        let image = load_image(bg)?;
        let highlighted = adjust_brightness(&image, 1.5);

        Ok(Self{
            rect,
            image,
            highlighted,
            checked,
            mouse_inside: Cell::new(false),
        })
    }
}

impl Widget for Checkbox {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_mouse_button_down(&self, button: Mouse, x: u16, y: u16) -> Option<Effect> {
        if self.rect.contains_point(Point::new(x as i32, y as i32)) && button == Mouse::Left {
            // sound->play(L"click.wav"); TODO
            self.checked.set(!self.checked.get());
            Some(Effect::Redraw(vec![self.rect]))
        } else {
            None
        }
    }

    fn on_mouse_move(&self, x: u16, y: u16) -> Option<Effect> {
        let to_highlight = self.rect.contains_point(Point::new(x as i32, y as i32));
        if self.mouse_inside.get() != to_highlight {
            self.mouse_inside.set(to_highlight);
            Some(Effect::Redraw(vec![self.rect]))
        } else {
            None
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
            draw_text(surface, "X", text_font()?, Color::RGB(255, 255, 255), true, self.get_rect(), HorizontalAlign::Center, VerticalAlign::Middle)?;
        }
        Ok(())
    }
}
