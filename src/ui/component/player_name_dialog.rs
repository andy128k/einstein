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
use ui::component::dialog::DialogResult;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::main_loop::main_loop;
use resources::background::BLUE_PATTERN;
use resources::messages::{get_messages, Messages};

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
        text: messages.enter_name.to_string(),
        rect: Rect::new(180, 300, 150, 26),
        color: yellow,
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(InputField::new(Rect::new(340, 300, 280, 26), BLUE_PATTERN, player_name.clone(), 20)?));

    {
        let state_weak = Rc::downgrade(&state);
        container.add(Box::new(new_dialog_button(Rect::new(348, 340, 90, 25), yellow, BLUE_PATTERN, messages.ok,
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

pub fn ask_player_name(surface: &Surface, default_name: &str) -> Result<DialogResult<String>> {
    let messages = get_messages();
    let container = new_player_name(default_name, messages)?;
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
