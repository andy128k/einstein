use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use failure::err_msg;
use sdl::event::{Key};
use sdl::video::{Surface, SurfaceFlag};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use ui::widget::widget::*;
use ui::utils::{draw_bevel, draw_text, HorizontalAlign, VerticalAlign, rect2_to_rect};
use resources::fonts::text_font;
use error::*;

pub struct InputField {
    rect: Rect,
    cursor: Surface,
    max_len: usize,
    text: Rc<RefCell<String>>,
    cursor_pos: Cell<usize>,
    last_cursor: Cell<Instant>,
    cursor_visible: Cell<bool>,
}

impl InputField {
    pub fn new(rect: Rect, text: &str, max_len: usize) -> Result<Self> {
        let cursor = Surface::new(&[SurfaceFlag::SWSurface], 2, rect.height() as isize - 8, 32, 0, 0, 0, 0).map_err(err_msg)?;
        cursor.fill(::sdl::video::Color::RGB(33, 33, 33));

        Ok(Self {
            rect,
            cursor,
            max_len,
            text: Rc::new(RefCell::new(text.to_string())),
            cursor_pos: Cell::new(text.len()),
            last_cursor: Cell::new(Instant::now()),
            cursor_visible: Cell::new(true),
        })
    }

    fn move_cursor(&self, pos: usize) {
        self.cursor_pos.set(pos);
        self.last_cursor.set(Instant::now());
        self.cursor_visible.set(true);
    }

    fn on_key_down(&self, key: Key, ch: u16) -> EventReaction<String> {
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
                EventReaction::Action(self.text.borrow().clone())
            },
            (Key::Left, _) => {
                if cursor_pos > 0 {
                    self.move_cursor(cursor_pos - 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::Redraw
            },
            (Key::Right, _) => {
                if cursor_pos < text_len {
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::Redraw
            },
            (Key::Home, _) => {
                self.move_cursor(0);
                EventReaction::Redraw
            },
            (Key::End, _) => {
                self.move_cursor(text_len);
                EventReaction::Redraw
            },
            (Key::Delete, _) => {
                if cursor_pos < text_len {
                    self.text.borrow_mut().remove(cursor_pos);
                }
                self.move_cursor(cursor_pos);
                EventReaction::Action(self.text.borrow().clone())
            },
            (_, ch) if ch > 32 => {
                if text_len < self.max_len {
                    self.text.borrow_mut().insert(cursor_pos, ::std::char::from_u32(ch as u32).unwrap());
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::Action(self.text.borrow().clone())
            },
            _ => EventReaction::NoOp,
        }
    }

    fn on_tick(&self) -> EventReaction<String> {
        let now = Instant::now();
        if now - self.last_cursor.get() > Duration::from_millis(1000) {
            self.cursor_visible.set(!self.cursor_visible.get());
            self.last_cursor.set(now);
            EventReaction::Redraw
        } else {
            EventReaction::NoOp
        }
    }
}

impl Widget<String> for InputField {
    fn on_event(&self, event: &Event) -> EventReaction<String> {
        match *event {
            Event::KeyDown(key, ch) => self.on_key_down(key, ch),
            Event::Tick => self.on_tick(),
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_bevel(surface, self.rect, false, 1);

        let window_rect = self.rect;
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
            surface.blit_at(&self.cursor, window_rect.left() as i16 + pos as i16, window_rect.top() as i16 + 4);
        }

        surface.set_clip_rect(None);

        Ok(())
    }
}
