use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use std::ffi::{CStr, CString};
use libc::memcpy;
use sdl;
use sdl::video::{Surface};
use sdl::event::Key;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::input_field::*;
use ui::widget::window::*;
use ui::widget::container::*;
use ui::widget::page_view::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::page_layout::{Page, PagesBuilder};
use ui::background::BLUE_PATTERN;
use locale::get_language;

struct Messages {
    title: &'static str,
    ok: &'static str,
    cancel: &'static str,
}

const MESSAGES_EN: Messages = Messages {
    title: "Enter your name:",
    ok: "OK",
    cancel: "Cancel",
};

const MESSAGES_RU: Messages = Messages {
    title: "Введите ваше имя:",
    ok: "OK",
    cancel: "Отмена",
};

pub enum DialogResult<T> {
    Ok(T),
    Cancel,
    Quit,
}

struct PlayerNameState {
    ok: bool,
    name: Rc<RefCell<String>>,
}

type PlayerNameStatePtr = Rc<RefCell<PlayerNameState>>;

fn new_player_name(name: &str, messages: &Messages) -> Result<Container<PlayerNameStatePtr>> {
    let rect = Rect::new(170, 280, 460, 100);
    let yellow = Color::RGB(255, 255, 0);

    let player_name = Rc::new(RefCell::new(name.to_string()));

    let state = Rc::new(RefCell::new(PlayerNameState {
        ok: false,
        name: player_name.clone(),
    }));

    let mut container = Container::new(rect, state.clone());

    container.add(Box::new(Window::new(rect, BLUE_PATTERN)?));
    container.add(Box::new(Label {
        font: text_font()?,
        text: messages.title.to_string(),
        rect: Rect::new(180, 300, 150, 26),
        color: yellow,
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Middle,
        shadow: true,
    }));

    container.add(Box::new(InputField::new(Rect::new(340, 300, 280, 26), BLUE_PATTERN, player_name.clone(), 20)?));

    {
        let state_weak = Rc::downgrade(&state);
        container.add(Box::new(Button::new(Rect::new(348, 340, 90, 25), yellow, BLUE_PATTERN, messages.ok,
            Some(Key::Return),
            move || {
                if let Some(state) = state_weak.upgrade() {
                    state.borrow_mut().ok = true;
                }
                Some(Effect::Terminate)
            }
        )?));
    }

    Ok(container)
}

fn ask_player_name(surface: &Surface, default_name: &str) -> Result<DialogResult<String>> {
    let messages = if get_language() == Some("ru".to_string()) {
        &MESSAGES_RU
    } else {
        &MESSAGES_EN
    };
    let container = new_player_name(default_name, &messages)?;
    let quit = main_loop(surface, &container)?;
    if quit {
        return Ok(DialogResult::Quit);
    }
    if container.private.borrow().ok {
        let state = container.private.borrow();
        let name = state.name.borrow().clone();
        Ok(DialogResult::Ok(name))
    } else {
        Ok(DialogResult::Ok(default_name.to_string()))
    }
}
