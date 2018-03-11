use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use error::*;
use ui::widget::widget::*;
use ui::utils::{tiled_image, adjust_brightness, draw_bevel, draw_text, HorizontalAlign, VerticalAlign};
use ui::fonts::*;

pub struct Button {
    text: String,
    rect: Rect,
    color: Color,
    image: Surface,
    highlighted_image: Surface,
    highlighted: Cell<bool>,
    key: Option<Key>,
    action: Box<Fn() -> Option<Effect>>
}

pub fn beveled_button_backgrounds(image: &[u8], width: u32, height: u32) -> Result<(Surface, Surface)> {
    let mut image = tiled_image(image, width as u16, height as u16)?;

    image.lock();
    {
        let inner_rect = Rect::new(1, 1, width - 2, height - 2);
        draw_bevel(&mut image, inner_rect, true, 1);

        let outer_rect = Rect::new(0, 0, width, height);
        draw_bevel(&mut image, outer_rect, false, 1);
    }
    image.unlock();

    let highlighted_image = adjust_brightness(&mut image, 1.5, false);

    Ok((image, highlighted_image))
}

impl Button {
    pub fn new1<A: Fn() -> Option<Effect> + 'static>(rect: Rect, bg: Surface, highlighted_bg: Surface, color: Color, text: &str, key: Option<Key>, action: A) -> Button {
        Self {
            text: text.to_string(),
            rect,
            color,
            image: bg,
            highlighted_image: highlighted_bg,
            highlighted: Cell::new(false),
            key,
            action: Box::new(action)
        }
    }

    pub fn new<A: Fn() -> Option<Effect> + 'static>(rect: Rect, color: Color, image: &[u8], text: &str, key: Option<Key>, action: A) -> Result<Button> {
        let (bg, highlighted_bg) = beveled_button_backgrounds(image, rect.width(), rect.height())?;
        Ok(Self::new1(rect, bg, highlighted_bg, color, text, key, action))
    }
}

impl Widget for Button {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_key_down(&self, key: Key, _ch: u16) -> Option<Effect> {
        if self.key == Some(key) {
            (*self.action)()
        } else {
            None
        }
    }

    fn on_mouse_button_down(&self, button: Mouse, x: u16, y: u16) -> Option<Effect> {
        if self.rect.contains_point(Point::new(x as i32, y as i32)) && button == Mouse::Left {
            // sound->play(L"click.wav"); TODO
            (*self.action)()
        } else {
            None
        }
    }

    fn on_mouse_move(&self, x: u16, y: u16) -> Option<Effect> {
        let to_highlight = self.rect.contains_point(Point::new(x as i32, y as i32));
        if self.highlighted.get() != to_highlight {
            self.highlighted.set(to_highlight);
            Some(Effect::Redraw(vec![self.rect]))
        } else {
            None
        }
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        let image = if self.highlighted.get() {
            &self.highlighted_image
        } else {
            &self.image
        };
        surface.blit_at(image, self.rect.left() as i16, self.rect.top() as i16);

        let rect = self.get_rect();
        let text_rect = rect;
        draw_text(surface, &self.text, button_font()?, self.color, true, text_rect, HorizontalAlign::Center, VerticalAlign::Middle)?;

        Ok(())
    }
}
