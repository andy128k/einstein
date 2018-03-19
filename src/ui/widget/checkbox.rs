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

pub struct Checkbox {
    rect: Rect,
    image: Surface,
    highlighted: Surface,
    checked: Rc<Cell<bool>>,
    mouse_inside: Cell<bool>,
}

impl Checkbox {
    pub fn new(rect: Rect, bg: &[u8], checked: Rc<Cell<bool>>) -> Result<Self> {
        let mut image = tiled_image(bg, rect.width() as u16, rect.height() as u16)?;

        image.lock();
        {
            let inner_rect = Rect::new(1, 1, rect.width() - 2, rect.height() - 2);
            draw_bevel(&mut image, inner_rect, true, 1);

            let outer_rect = Rect::new(0, 0, rect.width(), rect.height());
            draw_bevel(&mut image, outer_rect, false, 1);
        }
        image.unlock();

        let highlighted = adjust_brightness(&mut image, 1.5, false);

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
        surface.blit_at(image, self.rect.left() as i16, self.rect.top() as i16);
        if self.checked.get() {
            draw_text(surface, "X", text_font()?, Color::RGB(255, 255, 255), true, self.get_rect(), HorizontalAlign::Center, VerticalAlign::Middle)?;
        }
        Ok(())
    }
}
