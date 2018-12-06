use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::{Cell};
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::context::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;
use crate::audio::Audio;
use crate::error::*;

pub struct InputField {
    size: Size,
    max_len: usize,
    text: Rc<RefCell<String>>,
    cursor_pos: Cell<usize>,
    last_cursor: Cell<Instant>,
    cursor_visible: Cell<bool>,
}

impl InputField {
    pub fn new(size: Size, text: &str, max_len: usize) -> Result<Self> {
        Ok(Self {
            size,
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

    fn on_key_down(&self, key: Keycode) -> EventReaction<String> {
        let cursor_pos = self.cursor_pos.get();
        let text_len = self.text.borrow().len();
        match key {
            Keycode::Backspace => {
                if cursor_pos > 0 {
                    self.text.borrow_mut().remove(cursor_pos - 1);
                    self.move_cursor(cursor_pos - 1); // TODO. char indicies
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::update_and_action(self.text.borrow().clone())
            },
            Keycode::Left => {
                if cursor_pos > 0 {
                    self.move_cursor(cursor_pos - 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::update()
            },
            Keycode::Right => {
                if cursor_pos < text_len {
                    self.move_cursor(cursor_pos + 1);
                } else {
                    self.move_cursor(cursor_pos);
                }
                EventReaction::update()
            },
            Keycode::Home => {
                self.move_cursor(0);
                EventReaction::update()
            },
            Keycode::End => {
                self.move_cursor(text_len);
                EventReaction::update()
            },
            Keycode::Delete => {
                if cursor_pos < text_len {
                    self.text.borrow_mut().remove(cursor_pos);
                }
                self.move_cursor(cursor_pos);
                EventReaction::update_and_action(self.text.borrow().clone())
            },
            _ => EventReaction::empty(),
        }
    }

    fn on_text_input(&self, text: &str) -> EventReaction<String> {
        let cursor_pos = self.cursor_pos.get();
        let text_len = self.text.borrow().len();
        for ch in text.chars() {
            if text_len < self.max_len {
                self.text.borrow_mut().insert(cursor_pos, ::std::char::from_u32(ch as u32).unwrap());
                self.move_cursor(cursor_pos + 1);
            } else {
                self.move_cursor(cursor_pos);
            }
        }
        if !text.is_empty() {
            EventReaction::update_and_action(self.text.borrow().clone())
        } else {
            EventReaction::empty()
        }
    }

    fn on_tick(&self) -> EventResult<String> {
        let now = Instant::now();
        if now - self.last_cursor.get() > Duration::from_millis(1000) {
            self.cursor_visible.set(!self.cursor_visible.get());
            self.last_cursor.set(now);
            Ok(EventReaction::update())
        } else {
            Ok(EventReaction::empty())
        }
    }
}

impl Widget<String> for InputField {
    fn get_size(&self) -> Size { self.size }

    fn on_event(&mut self, event: &Event, _resource_manager: &dyn ResourceManager, _audio: &Audio) -> EventResult<String> {
        match *event {
            Event::KeyDown(key) => Ok(self.on_key_down(key)),
            Event::TextInput(ref text) => Ok(self.on_text_input(text)),
            Event::Tick => self.on_tick(),
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        let font_size = FontSize::TEXT;

        let mut brick = Brick::new(self.get_size().width, self.get_size().height)
            .border(Border::Sunken)
            .text(Text::new(&self.text.borrow()).font_size(font_size).color(Color::RGB(255, 255, 0)).shadow().halign(HorizontalAlign::Left));

        if self.cursor_visible.get() {
            let cursor_pos = self.cursor_pos.get();
            let pos = if cursor_pos > 0 {
                resource_manager.font(font_size.0).size_of(&self.text.borrow()[0..cursor_pos]).unwrap().0
            } else {
                0
            };
            let cursor = Brick::new(2, self.get_size().height - 8).background(Background::Color(Color::RGB(33, 33, 33)));
            brick.push(pos as u32, 4, cursor);
        }

        brick
    }
}
