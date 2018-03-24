use std::rc::Rc;
use debug_cell::RefCell;
use sdl;
use sdl::video::{Surface};
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::window::*;
use ui::widget::container::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::background::BLUE_PATTERN;
use locale::get_language;
use storage::{Storage, Scores};
use util::time::sec_to_str;

struct Messages {
    title: &'static str,
    ok: &'static str,
}

const MESSAGES_EN: Messages = Messages {
    title: "Hall Of Fame",
    ok: "OK",
};

const MESSAGES_RU: Messages = Messages {
    title: "Доска почета",
    ok: "OK",
};

fn create_topscores(scores: &Scores, messages: &Messages, highlight: Option<usize>) -> Result<Container<()>> {
    let rect = Rect::new(240, 125, 320, 350);

    let mut container = Container::new(rect, ());

    let window = Window::new(rect.clone(), BLUE_PATTERN)?;

    let title = Label {
        font: title_font()?,
        text: messages.title.to_string(),
        rect: Rect::new(250, 135, 300, 40),
        color: Color::RGB(255, 255, 0),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
        shadow: true
    };

    container.add(Box::new(window));
    container.add(Box::new(title));

    let mut pos = 175;
    for (i, score) in scores.0.iter().enumerate() {
        let color = if highlight == Some(i) {
            Color::RGB(255, 255, 0)
        } else {
            Color::RGB(255, 255, 255)
        };

        container.add(Box::new(Label {
            font: text_font()?,
            text: format!("{}.", i + 1),
            rect: Rect::new(250, pos, 30, 25),
            color,
            horizontal_align: HorizontalAlign::Right,
            vertical_align: VerticalAlign::Middle,
            shadow: true
        }));

        container.add(Box::new(Label {
            font: text_font()?,
            text: score.name.clone(),
            rect: Rect::new(290, pos, 160, 25),
            color,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            shadow: true
        }));

        container.add(Box::new(Label {
            font: text_font()?,
            text: sec_to_str(score.score),
            rect: Rect::new(460, pos, 80, 25),
            color,
            horizontal_align: HorizontalAlign::Right,
            vertical_align: VerticalAlign::Middle,
            shadow: true
        }));

        pos += 25;
    }

    let close = Button::new(Rect::new(355, 435, 90, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, messages.ok,
        Some(Key::Escape),
        || Some(Effect::Terminate)
    )?;

    container.add(Box::new(close));

    Ok(container)
}

pub fn show_scores(surface: &Surface, scores: &Scores, highlight: Option<usize>) -> Result<bool> {
    let messages = if get_language() == Some("ru".to_string()) {
        &MESSAGES_RU
    } else {
        &MESSAGES_EN
    };
    let topscores = create_topscores(scores, messages, highlight)?;
    main_loop(surface, &topscores)
}

#[no_mangle]
pub extern fn ein_show_scores(surface_ptr: * mut sdl::video::ll::SDL_Surface, storage_ptr: *const Rc<RefCell<Storage>>) -> ::libc::c_int {
    let surface = sdl::video::Surface { raw: surface_ptr, owned: false };
    let storage: &Rc<RefCell<Storage>> = unsafe { &* storage_ptr };
    show_scores(&surface, &storage.borrow().scores, None).unwrap() as i32
}
