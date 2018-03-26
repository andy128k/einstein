use std::mem;
use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl;
use sdl::video::{Surface};
use sdl::event::Key;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::widget::container::*;
use ui::widget::page_view::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::page_layout::{Page, PagesBuilder};
use ui::background::BLUE_PATTERN;
use ui::component::game::GamePrivate;
use ui::component::game_name_dialog::*;
use ui::component::dialog::DialogResult;
use locale::get_language;
use storage::{Storage, MAX_SLOTS, SavedGame};

struct ListWindowState {
    result: DialogResult<GamePrivate>,
}

type ListWindowStatePtr = Rc<RefCell<ListWindowState>>;

fn create_list_window(saved_games: &[Option<SavedGame>], title: &str) -> Result<Container<ListWindowStatePtr>> {
    let rect = Rect::new(250, 90, 300, 420);
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(ListWindowState {
        result: DialogResult::Cancel,
    }));

    let mut container = Container::new(rect, state.clone());

    container.add(Box::new(Window::new(rect, BLUE_PATTERN)?));
    container.add(Box::new(Title {
        text: title.to_string(),
        rect: Rect::new(250, 95, 300, 40),
    }));

    let close = Button::new(Rect::new(360, 470, 80, 25), yellow, BLUE_PATTERN, "close", // TODO i18n
        Some(Key::Escape),
        || Some(Effect::Terminate)
    )?;

    for (i, game) in saved_games.iter().enumerate() {
        let (label, default_name): (String, String) = match *game {
            Some(ref g) => (g.name.to_string(), g.name.to_string()),
            None => (
                "-- empty --".to_string(), // msg(L"empty")
                format!("game {}", i + 1)
            )
        };

        let game2: Option<SavedGame> = (*game).clone();
        let state_weak = Rc::downgrade(&state);
        container.add(Box::new(Button::new(Rect::new(260, 150 + (i as i32) * 30, 280, 25), yellow, BLUE_PATTERN, &label,
            None,
            move || {
                if let Some(ref game3) = game2 {
                    if let Some(state) = state_weak.upgrade() {
                        state.borrow_mut().result = DialogResult::Ok(game3.game.clone());
                    }
                    Some(Effect::Terminate)
                } else {
                    None
                }
            }
        )?));
    }

    Ok(container)
}

pub fn load_game(surface: Rc<Surface>, storage: &Storage) -> Result<Option<GamePrivate>> {
    let container = create_list_window(&storage.saved_games, "Load Game" /*msg(L"loadGame")*/)?;

    let quit = main_loop(&surface, &container)?;
    if quit {
        ::std::process::exit(0);
    }

    let result = mem::replace(&mut container.private.borrow_mut().result, DialogResult::Cancel);
    match result {
        DialogResult::Ok(game) => {
            Ok(Some(game))
        },
        DialogResult::Cancel => Ok(None),
        DialogResult::Quit => {
            ::std::process::exit(0);
        }
    }
}
