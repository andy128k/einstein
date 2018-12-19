use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::{Cell};
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::context::Context;
use crate::resources::manager::ResourceManager;

pub struct InputField {
    size: Size,
    max_len: usize,
    text: Rc<RefCell<String>>,
    cursor_pos: Cell<usize>,
    last_cursor: Cell<Instant>,
    cursor_visible: Cell<bool>,
}

impl InputField {
    pub fn new(size: Size, text: &str, max_len: usize) -> Self {
        Self {
            size,
            max_len,
            text: Rc::new(RefCell::new(text.to_string())),
            cursor_pos: Cell::new(text.chars().count()),
            last_cursor: Cell::new(Instant::now()),
            cursor_visible: Cell::new(true),
        }
    }

    fn move_cursor(&self, pos: usize) {
        self.cursor_pos.set(pos);
        self.last_cursor.set(Instant::now());
        self.cursor_visible.set(true);
    }

    fn on_key_down(&self, key: Keycode) -> EventReaction<String> {
        let cursor_pos = self.cursor_pos.get();
        let text_len = self.text.borrow().chars().count();
        match key {
            Keycode::Backspace => {
                if cursor_pos > 0 {
                    let t = delete_char(&*self.text.borrow(), cursor_pos - 1);
                    *self.text.borrow_mut() = t;
                    self.move_cursor(cursor_pos - 1);
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
                    let t = delete_char(&*self.text.borrow(), cursor_pos);
                    *self.text.borrow_mut() = t;
                }
                self.move_cursor(cursor_pos);
                EventReaction::update_and_action(self.text.borrow().clone())
            },
            _ => EventReaction::empty(),
        }
    }

    fn on_text_input(&self, text: &str) -> EventReaction<String> {
        for ch in text.chars() {
            let cursor_pos = self.cursor_pos.get();
            let text_len = self.text.borrow().chars().count();
            if text_len < self.max_len {
                let t = insert_char(&*self.text.borrow(), cursor_pos, ch);
                *self.text.borrow_mut() = t;
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

    fn on_event(&mut self, event: &Event, _context: &dyn Context) -> EventResult<String> {
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
                resource_manager.font(font_size.0).size_of(&self.text.borrow().chars().take(cursor_pos).collect::<String>()).unwrap().0
            } else {
                0
            };
            let cursor = Brick::new(2, self.get_size().height - 8).background(Background::Color(Color::RGB(33, 33, 33)));
            brick.push(pos as u32, 4, cursor);
        }

        brick
    }
}

fn insert_char(text: &str, index: usize, ich: char) -> String {
    let mut result: String = text.chars().take(index).collect();
    result.push(ich);
    result.extend(text.chars().skip(index));
    result
}

fn delete_char(text: &str, index: usize) -> String {
    let mut result: String = text.chars().take(index).collect();
    result.extend(text.chars().skip(index + 1));
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ui::context::{MainLoopQuit, Context};
    use crate::error::*;
    use crate::audio::Audio;

    struct ContextMock;

    impl Context for ContextMock {
        fn resource_manager(&self) -> &dyn ResourceManager { unreachable!() }
        fn audio(&self) -> &dyn Audio { unreachable!() }
        fn main_loop(&self, _widget: &mut dyn Widget<MainLoopQuit>) -> Result<()> { unreachable!() }
    }

    #[test]
    fn test_insert() {
        assert_eq!(insert_char("", 0, 'A'), "A");
        assert_eq!(insert_char("Bb", 1, 'o'), "Bob");
    }

    #[test]
    fn test_delete() {
        assert_eq!(delete_char("", 0), "");
        assert_eq!(delete_char("A", 0), "");
        assert_eq!(delete_char("Latte", 2), "Late");
    }

    #[test]
    fn test_input() {
        let context = ContextMock;
        let mut field = InputField::new(Size::new(300, 40), "", 5);

        field.on_event(&Event::TextInput("A".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "A");

        field.on_event(&Event::KeyDown(Keycode::Backspace), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "");

        field.on_event(&Event::TextInput("B".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "B");

        field.on_event(&Event::TextInput("o".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "Bo");

        field.on_event(&Event::TextInput("ы".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "Boы");

        field.on_event(&Event::TextInput("y".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "Boыy");

        field.on_event(&Event::KeyDown(Keycode::Left), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "Boыy");

        field.on_event(&Event::KeyDown(Keycode::Backspace), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "Boy");

        field.on_event(&Event::KeyDown(Keycode::Backspace), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "By");

        field.on_event(&Event::KeyDown(Keycode::Backspace), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "y");

        field.on_event(&Event::KeyDown(Keycode::Right), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "y");

        field.on_event(&Event::KeyDown(Keycode::Right), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "y");

        field.on_event(&Event::KeyDown(Keycode::Right), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "y");

        field.on_event(&Event::TextInput("e".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "ye");

        field.on_event(&Event::TextInput("s".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "yes");

        field.on_event(&Event::KeyDown(Keycode::Left), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "yes");

        field.on_event(&Event::KeyDown(Keycode::Left), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "yes");

        field.on_event(&Event::KeyDown(Keycode::Left), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "yes");

        field.on_event(&Event::KeyDown(Keycode::Delete), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "es");

        field.on_event(&Event::KeyDown(Keycode::Delete), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "s");

        field.on_event(&Event::KeyDown(Keycode::Delete), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "");

        field.on_event(&Event::TextInput("4".to_owned()), &context).unwrap();
        assert_eq!(&*field.text.borrow(), "4");
    }
}
