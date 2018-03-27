use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
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
use ui::main_loop::main_loop;
use ui::page_layout::{Page, PagesBuilder};
use ui::component::game::GamePrivate;
use ui::component::game_name_dialog::*;
use ui::component::dialog::DialogResult;
use resources::fonts::*;
use resources::background::BLUE_PATTERN;
use resources::messages::{get_messages, Messages};
use storage::{Storage, MAX_SLOTS, SavedGame};

struct ListWindowState {
    result: DialogResult<(usize, String)>,
    selected: Box<Fn(&str) -> DialogResult<String>>,
}

type ListWindowStatePtr = Rc<RefCell<ListWindowState>>;

fn create_list_window<F>(saved_games: &[Option<SavedGame>], messages: &Messages, selected: F) -> Result<Container<ListWindowStatePtr>>
    where
        F: Fn(&str) -> DialogResult<String> + 'static
{
    let rect = Rect::new(250, 90, 300, 420);
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(ListWindowState {
        result: DialogResult::Cancel,
        selected: Box::new(selected),
    }));

    let mut container = Container::new(rect, state.clone());

    container.add(Box::new(Window::new(rect, BLUE_PATTERN)?));
    container.add(Box::new(Title {
        text: messages.save_game.to_string(),
        rect: Rect::new(250, 95, 300, 40),
    }));

    let close = Button::new(Rect::new(360, 470, 80, 25), yellow, BLUE_PATTERN, messages.close,
        Some(Key::Escape),
        || Some(Effect::Terminate)
    )?;

    for (i, game) in saved_games.iter().enumerate() {
        let (label, default_name): (String, String) = match *game {
            Some(ref g) => (g.name.to_string(), g.name.to_string()),
            None => (
                messages.empty.to_string(),
                format!("{} {}", messages.default_game_name, i + 1)
            )
        };

        let state_weak = Rc::downgrade(&state);
        container.add(Box::new(Button::new(Rect::new(260, 150 + (i as i32) * 30, 280, 25), yellow, BLUE_PATTERN, &label,
            None,
            move || {
                if let Some(state) = state_weak.upgrade() {
                    let result = (state.borrow().selected)(&default_name);
                    state.borrow_mut().result = match result {
                        DialogResult::Ok(name) => DialogResult::Ok((i, name)),
                        DialogResult::Cancel => DialogResult::Cancel,
                        DialogResult::Quit => DialogResult::Quit
                    }
                }
                Some(Effect::Terminate)
            }
        )?));
    }

    Ok(container)
}

pub fn save_game(surface: Rc<Surface>, storage: &mut Storage, game: &GamePrivate) -> Result<()> {
    let messages = get_messages();

    let container = {
        let surface_weak = Rc::downgrade(&surface);
        create_list_window(&storage.saved_games, messages,
            move |default_name| {
                let surface = surface_weak.upgrade().unwrap();
                ask_game_name(&surface, &default_name).unwrap()
            }
        )?
    };

    let quit = main_loop(&surface, &container)?;
    if quit {
        ::std::process::exit(0);
    }

    let private = container.private.borrow();
    match private.result {
        DialogResult::Ok((index, ref name)) => {
            storage.saved_games[index] = Some(SavedGame {
                name: name.to_owned(),
                game: game.clone()
            });
            Ok(())
        },
        DialogResult::Cancel => Ok(()),
        DialogResult::Quit => {
            ::std::process::exit(0);
        }
    }
}
