use std::rc::Rc;
use std::cell::Cell;
use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::widget::dialog_button::new_dialog_button;
use ui::widget::container::*;
use ui::main_loop::main_loop;
use ui::component::dialog::DialogResult;
use resources::background::RED_PATTERN;
use resources::messages::{Messages, get_messages};

#[derive(Clone, Copy)]
pub enum Choice {
    StartNew,
    TryAgain
}

fn new_failure_dialog(messages: &Messages) -> Result<Container<Rc<Cell<Option<Choice>>>>> {
    let state = Rc::new(Cell::new(None));

    let rect = Rect::new(220, 240, 360, 140);

    let mut container = Container::new(rect, state.clone());

    container.add(Box::new(Window::new(rect, RED_PATTERN)?));
    container.add(Box::new(Title { rect: Rect::new(250, 230, 300, 100), text: messages.loose.to_string() }));

    container.add(Box::new({
        let state2 = state.clone();
        new_dialog_button(Rect::new(250, 340, 90, 25), RED_PATTERN, messages.start_new, None,
            move || {
                state2.set(Some(Choice::StartNew));
                Some(Effect::Terminate)
            }
        )?
    }));
    container.add(Box::new({
        let state2 = state.clone();
        new_dialog_button(Rect::new(350, 340, 90, 25), RED_PATTERN, messages.try_again, None,
            move || {
                state2.set(Some(Choice::TryAgain));
                Some(Effect::Terminate)
            }
        )?
    }));
    container.add(Box::new({
        new_dialog_button(Rect::new(450, 340, 90, 25), RED_PATTERN, messages.exit, None,
            || Some(Effect::Terminate)
        )?
    }));

    Ok(container)
}

pub fn show_failure_dialog(surface: &Surface) -> Result<DialogResult<Choice>> {
    let dlg = new_failure_dialog(get_messages())?;
    let quit = main_loop(surface, &dlg)?;
    if quit {
        return Ok(DialogResult::Quit);
    }

    let result = match dlg.private.get() {
        Some(choice) => DialogResult::Ok(choice),
        None => DialogResult::Cancel,
    };

    Ok(result)
}
