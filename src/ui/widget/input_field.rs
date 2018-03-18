use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use failure::err_msg;
use sdl::event::{Key, Mouse};
use sdl::video::{Surface, SurfaceFlag};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use ui::widget::widget::*;
use ui::utils::{tiled_image, draw_bevel, draw_text, HorizontalAlign, VerticalAlign, rect2_to_rect};
use ui::fonts::text_font;
use error::*;

pub struct InputField {
    rect: Rect,
    background: Surface,
    cursor: Surface,
    max_len: usize,
    text: Rc<RefCell<String>>,
    cursor_pos: Cell<usize>,
    last_cursor: Cell<Instant>,
    cursor_visible: Cell<bool>,
}

impl InputField {
    pub fn new(rect: Rect, bg: &[u8], text: Rc<RefCell<String>>, max_len: usize) -> Result<Self> {
        let cursor = Surface::new(&[SurfaceFlag::SWSurface], 2, rect.height() as isize - 4, 32, 0, 0, 0, 0).map_err(err_msg)?;
        cursor.fill(::sdl::video::Color::RGB(255, 255, 0));

        let mut win = tiled_image(bg, rect.width() as u16, rect.height() as u16)?;

        win.lock();
        let mut bounding_rect = rect;
        bounding_rect.reposition((0, 0));
        draw_bevel(&mut win, bounding_rect, false, 1);
        win.unlock();

        let background = win.display_format().map_err(err_msg)?;

        let text_len = text.borrow().len();

        Ok(Self {
            rect,
            background,
            cursor,
            max_len,
            text,
            cursor_pos: Cell::new(text_len),
            last_cursor: Cell::new(Instant::now()),
            cursor_visible: Cell::new(true),
        })
    }

    fn move_cursor(&self, pos: usize) {
        self.cursor_pos.set(pos);
        self.last_cursor.set(Instant::now());
        self.cursor_visible.set(true);
    }
}

impl Widget for InputField {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_tick(&self) -> Option<Effect> {
        let now = Instant::now();
        if now - self.last_cursor.get() > Duration::from_millis(1000) {
            self.cursor_visible.set(!self.cursor_visible.get());
            self.last_cursor.set(now);
            Some(Effect::Redraw(vec![self.get_rect()]))
        } else {
            None
        }
    }

    fn on_key_down(&self, key: Key, ch: u16) -> Option<Effect> {
        let redraw_all = Some(Effect::Redraw(vec![self.get_rect()]));
        let cursor_pos = self.cursor_pos.get();
        let text_len = self.text.borrow().len();
        match (key, ch) {
            (Key::Backspace, _) => {
                if cursor_pos > 0 {
                    self.text.borrow_mut().remove(cursor_pos - 1);
                    self.move_cursor(cursor_pos - 1); // TODO. char indicies
                } else {
                    self.move_cursor(cursor_pos);
                }
                redraw_all
            },
            (Key::Left, _) => {
                if cursor_pos > 0 {
                    self.move_cursor(cursor_pos - 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                redraw_all
            },
            (Key::Right, _) => {
                if cursor_pos < text_len {
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                redraw_all
            },
            (Key::Home, _) => {
                self.move_cursor(0);
                redraw_all
            },
            (Key::End, _) => {
                self.move_cursor(text_len);
                redraw_all
            },
            (Key::Delete, _) => {
                if cursor_pos < text_len {
                    self.text.borrow_mut().remove(cursor_pos);
                }
                self.move_cursor(cursor_pos);
                redraw_all
            },
            (_, ch) if ch > 32 => {
                if text_len < self.max_len {
                    self.text.borrow_mut().insert(cursor_pos, ::std::char::from_u32(ch as u32).unwrap());
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                redraw_all
            },
            _ => None
        }
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        surface.blit_at(&self.background, self.rect.left() as i16, self.rect.top() as i16);

        let window_rect = self.get_rect();
        let rect = Rect::new(window_rect.left() + 1, window_rect.top() + 1, window_rect.width() - 2, window_rect.height() - 2);
        surface.set_clip_rect(Some(&rect2_to_rect(rect)));

        let font = text_font()?;
        draw_text(surface, &self.text.borrow(), font, Color::RGB(255, 255, 0), true, rect, HorizontalAlign::Left, VerticalAlign::Middle)?;

        if self.cursor_visible.get() {
            let cursor_pos = self.cursor_pos.get();
            let pos = if cursor_pos > 0 {
                font.size_of(&self.text.borrow()[0..cursor_pos])?.0
            } else {
                0
            };
            surface.blit_at(&self.cursor, window_rect.left() as i16 + pos as i16, window_rect.top() as i16 + 2);
        }

        surface.set_clip_rect(None);

        Ok(())
    }
}
