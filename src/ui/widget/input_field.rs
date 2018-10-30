use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::{Cell};
use cell::RefCell;
use sdl::event::{Key};
use sdl2::pixels::Color;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::*;
use ui::widget::brick::*;
use resources::manager::ResourceManager;
use resources::fonts::text_font;
use error::*;

pub struct InputField {
    rect: Rect,
    max_len: usize,
    text: Rc<RefCell<String>>,
    cursor_pos: Cell<usize>,
    last_cursor: Cell<Instant>,
    cursor_visible: Cell<bool>,
}

impl InputField {
    pub fn new(rect: Rect, text: &str, max_len: usize) -> Result<Self> {
        Ok(Self {
            rect,
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
                EventReaction::update_and_action(self.get_rect(), self.text.borrow().clone())
            },
            (Key::Left, _) => {
                if cursor_pos > 0 {
                    self.move_cursor(cursor_pos - 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::update(self.get_rect())
            },
            (Key::Right, _) => {
                if cursor_pos < text_len {
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::update(self.get_rect())
            },
            (Key::Home, _) => {
                self.move_cursor(0);
                EventReaction::update(self.get_rect())
            },
            (Key::End, _) => {
                self.move_cursor(text_len);
                EventReaction::update(self.get_rect())
            },
            (Key::Delete, _) => {
                if cursor_pos < text_len {
                    self.text.borrow_mut().remove(cursor_pos);
                }
                self.move_cursor(cursor_pos);
                EventReaction::update_and_action(self.get_rect(), self.text.borrow().clone())
            },
            (_, ch) if ch > 32 => {
                if text_len < self.max_len {
                    self.text.borrow_mut().insert(cursor_pos, ::std::char::from_u32(ch as u32).unwrap());
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::update_and_action(self.get_rect(), self.text.borrow().clone())
            },
            _ => EventReaction::empty(),
        }
    }

    fn on_tick(&self) -> EventResult<String> {
        let now = Instant::now();
        if now - self.last_cursor.get() > Duration::from_millis(1000) {
            self.cursor_visible.set(!self.cursor_visible.get());
            self.last_cursor.set(now);
            Ok(EventReaction::update(self.get_rect()))
        } else {
            Ok(EventReaction::empty())
        }
    }
}

impl Widget<String> for InputField {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&mut self, event: &Event) -> EventResult<String> {
        match *event {
            Event::KeyDown(key, ch) => Ok(self.on_key_down(key, ch)),
            Event::Tick => self.on_tick(),
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &mut ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_rect())
            .border(Border::Sunken)
            .text(Text::new(&self.text.borrow()).font_size(FontSize::Text).color(Color::RGB(255, 255, 0)).shadow().halign(HorizontalAlign::Left));

        if self.cursor_visible.get() {
            let cursor_pos = self.cursor_pos.get();
            let pos = if cursor_pos > 0 {
                text_font().unwrap().size_of(&self.text.borrow()[0..cursor_pos]).unwrap().0
            } else {
                0
            };
            let cursor_rect = Rect::new(pos as i32, 4, 2, self.get_rect().height() - 8);
            let cursor = Brick::new(cursor_rect).background(BackgroundPattern::Color(Color::RGB(33, 33, 33)));
            brick.push(cursor);
        }

        brick
    }
}
