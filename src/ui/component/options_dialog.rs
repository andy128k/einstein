use std::rc::Rc;
use std::cell::{Cell};
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
use ui::widget::checkbox::*;
use ui::widget::slider::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::widget::container::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::background::BLUE_PATTERN;
use locale::get_language;
use storage::Storage;

struct Options {
    fullscreen: Rc<Cell<bool>>,
    volume: Rc<Cell<f32>>,
    ok: Cell<bool>,
}

fn new_options_dialog(storage: &Storage) -> Result<Container<Rc<Options>>> {
    let rect = Rect::new(250, 170, 300, 260);

    let state = Rc::new(Options {
        fullscreen: Rc::new(Cell::new(storage.fullscreen)),
        volume: Rc::new(Cell::new(storage.volume as f32 / 100f32)),
        ok: Cell::new(false),
    });

    let mut container = Container::new(rect, state.clone());

    container.add(Box::new(Window::new(rect, BLUE_PATTERN)?));
    container.add(Box::new(Title {
        text: "Options".to_string(), // msg(L"options")
        rect: Rect::new(250, 175, 300, 40),
    }));

    container.add(Box::new(Checkbox::new(Rect::new(265, 260, 20, 20), BLUE_PATTERN, state.fullscreen.clone())?));

    container.add(Box::new(Label {
        text: "fullscreen".to_string(), // msg(L"fullscreen")
        rect: Rect::new(300, 260, 300, 20),
        color: Color::RGB(255, 255, 255),
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(Label {
        text: "volume".to_string(), // msg(L"volume")
        rect: Rect::new(265, 330, 300, 20),
        color: Color::RGB(255, 255, 255),
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(Slider::new(Rect::new(360, 332, 160, 16), BLUE_PATTERN, state.volume.clone())?));

    container.add(Box::new({
        let state_weak = Rc::downgrade(&state);
        Button::new(Rect::new(315, 390, 85, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, "ok", // i18n
            Some(Key::Return),
            move || {
                if let Some(state) = state_weak.upgrade() {
                    state.ok.set(true);
                }
                Some(Effect::Terminate)
            }
        )?
    }));

    container.add(Box::new(Button::new(Rect::new(405, 390, 85, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, "cancel", // i18n
        Some(Key::Escape),
        || Some(Effect::Terminate)
    )?));

    Ok(container)
}

pub fn show_options_window(surface: &Surface, storage: &mut Storage) -> Result<bool> {
    let container = new_options_dialog(storage)?;
    let quit = main_loop(&surface, &container)?;
    if container.private.ok.get() {
        storage.fullscreen = container.private.fullscreen.get();
        storage.volume = (container.private.volume.get() * 100f32) as u32;

        // screen->setMode(VideoMode(800, 600, 24, container.private.fullscreen.get()));
        // sound->setVolume(container.private.volume.get());
    }
    Ok(quit)
}
