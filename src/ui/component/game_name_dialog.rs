use std::rc::Rc;
use debug_cell::RefCell;
use sdl::video::{Surface};
use sdl::event::Key;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::dialog_button::*;
use ui::widget::input_field::*;
use ui::widget::window::*;
use ui::widget::container::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::component::dialog::DialogResult;
use ui::main_loop::main_loop;
use resources::background::BLUE_PATTERN;
use resources::messages::{get_messages, Messages};

struct GameNameState {
    ok: bool,
    name: Rc<RefCell<String>>,
}

type GameNameStatePtr = Rc<RefCell<GameNameState>>;

fn new_game_name(name: &str, messages: &Messages) -> Result<Container<GameNameStatePtr>> {
    let rect = Rect::new(170, 280, 460, 100);
    let yellow = Color::RGB(255, 255, 0);

    let game_name = Rc::new(RefCell::new(name.to_string()));

    let state = Rc::new(RefCell::new(GameNameState {
        ok: false,
        name: game_name.clone(),
    }));

    let mut container = Container::new(rect, state.clone());

    container.add(Box::new(Window::new(rect, BLUE_PATTERN)?));
    container.add(Box::new(Label {
        text: messages.enter_game.to_string(),
        rect: Rect::new(180, 300, 150, 26),
        color: yellow,
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(InputField::new(Rect::new(340, 300, 280, 26), BLUE_PATTERN, game_name.clone(), 20)?));

    {
        let state_weak = Rc::downgrade(&state);
        container.add(Box::new(new_dialog_button(Rect::new(310, 340, 80, 25), yellow, BLUE_PATTERN, messages.ok,
            Some(Key::Return),
            move || {
                if let Some(state) = state_weak.upgrade() {
                    state.borrow_mut().ok = true;
                }
                Some(Effect::Terminate)
            }
        )?));
    }

    container.add(Box::new(new_dialog_button(Rect::new(400, 340, 80, 25), yellow, BLUE_PATTERN, messages.cancel,
        Some(Key::Escape),
        || Some(Effect::Terminate)
    )?));

    Ok(container)
}

pub fn ask_game_name(surface: &Surface, default_name: &str) -> Result<DialogResult<String>> {
    let container = new_game_name(default_name, get_messages())?;
    let quit = main_loop(surface, &container)?;
    if quit {
        return Ok(DialogResult::Quit);
    }
    if container.private.borrow().ok {
        let state = container.private.borrow();
        let name = state.name.borrow().clone();
        Ok(DialogResult::Ok(name))
    } else {
        Ok(DialogResult::Cancel)
    }
}
