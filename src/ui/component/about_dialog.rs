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
use ui::widget::title::Title;
use ui::widget::container::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::background::BLUE_PATTERN;
use locale::get_language;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Messages {
    title: &'static str,
    name: &'static str,
    version: &'static str,
    copyright: &'static str,
    ok: &'static str,
}

const MESSAGES_EN: Messages = Messages {
    title: "About",
    name: "Einstein Puzzle",
    version: "version ",
    copyright: "Copyright (c) 2003-2005 Flowix Games",
    ok: "OK",
};

const MESSAGES_RU: Messages = Messages {
    title: "Об авторах",
    name: "Головоломка Эйнштейна",
    version: "версия ",
    copyright: "Copyright (c) 2003-2005 Flowix Games",
    ok: "OK",
};

fn create_about(messages: &Messages) -> Result<Container<()>> {
    let rect = Rect::new(220, 160, 360, 280);

    let mut container = Container::new(rect, ());

    container.add(Box::new(Window::new(rect.clone(), BLUE_PATTERN)?));

    container.add(Box::new(Title {
        text: messages.title.to_string(),
        rect: Rect::new(250, 165, 300, 40),
    }));

    container.add(Box::new(Label {
        text: messages.name.to_string(),
        rect: Rect::new(220, 240, 360, 20),
        color: Color::RGB(255, 255, 255),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(Label {
        text: format!("{}{}", messages.version, VERSION),
        rect: Rect::new(220, 260, 360, 20),
        color: Color::RGB(255, 255, 255),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(Label {
        text: messages.copyright.to_string(),
        rect: Rect::new(220, 280, 360, 20),
        color: Color::RGB(255, 255, 255),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(Label {
        text: "http://games.flowix.com".to_string(),
        rect: Rect::new(220, 330, 360, 20),
        color: Color::RGB(255, 255, 0),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
    }));

    let close = Button::new(Rect::new(360, 400, 80, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, messages.ok,
        Some(Key::Escape), // Return also
        || Some(Effect::Terminate)
    )?;

    container.add(Box::new(close));

    Ok(container)
}

pub fn show_about(surface: &Surface) -> Result<bool> {
    let messages = if get_language() == Some("ru".to_string()) {
        &MESSAGES_RU
    } else {
        &MESSAGES_EN
    };
    let about = create_about(messages)?;
    main_loop(surface, &about)
}
