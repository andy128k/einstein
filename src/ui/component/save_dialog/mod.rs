use std::convert::Into;
use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::Cell;
use debug_cell::RefCell;
use sdl::event::Key;
use sdl2::rect::Rect;
use error::*;
use ui::context::Context;
use ui::widget::widget::*;
use ui::widget::dialog_button::*;
use ui::widget::dialog::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::main_loop::{main_loop, ModalResult};
use ui::component::game::GamePrivate;
use ui::component::game_name_dialog::*;
use ui::component::dialog::DialogResult;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;
use storage::{Storage, SavedGame};

pub fn new_save_game_dialog(saved_games: &[Option<SavedGame>], messages: &'static Messages) -> Result<WidgetPtr<ModalResult<DialogResult<(usize, String)>>>> {
    let rect = Rect::new(250, 90, 300, 420);

    let mut container: Vec<WidgetPtr<ModalResult<DialogResult<(usize, String)>>>> = vec![];

    container.push(Box::new(
        InterceptWidget::default()
    ));

    container.push(Box::new(WidgetMapAction::no_action(
        Window::new(rect, BLUE_PATTERN)?
    )));

    container.push(Box::new(WidgetMapAction::no_action(
        Title {
            text: messages.save_game.to_string(),
            rect: Rect::new(250, 95, 300, 40),
        }
    )));

    let ask_name: Rc<RefCell<Option<(usize, String)>>> = Rc::new(RefCell::new(None));

    for (i, game) in saved_games.iter().enumerate() {
        let (label, default_name): (String, String) = match *game {
            Some(ref g) => (g.name.to_string(), g.name.to_string()),
            None => (
                messages.empty.to_string(),
                format!("{} {}", messages.default_game_name, i + 1)
            )
        };

        container.push(Box::new({
            let ask_name2 = ask_name.clone();
            WidgetMapAction::new(
                new_dialog_button(Rect::new(260, 150 + (i as i32) * 30, 280, 25), BLUE_PATTERN, &label, None, ())?,
                move |_| {
                    *ask_name2.borrow_mut() = Some((i, default_name.clone()));
                    EventReaction::Redraw
                }
            )
        }));
    }

    container.push(Box::new(
        new_dialog_button(Rect::new(360, 470, 80, 25), BLUE_PATTERN, messages.close, Some(Key::Escape), ModalResult(DialogResult::Cancel))?
    ));

    container.push(Box::new({
        let ask_name2 = ask_name.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                ask_name.clone(),
                move |&(index, ref name)| new_game_name(name, messages)
            ),
            move |result| {
                let index = ask_name2.borrow().as_ref().map(|p| p.0).unwrap();
                *ask_name2.borrow_mut() = None;
                match *result {
                    ModalResult(DialogResult::Ok(ref name)) => EventReaction::Action(ModalResult(DialogResult::Ok((index, name.to_string())))),
                    ModalResult(DialogResult::Cancel) => EventReaction::Action(ModalResult(DialogResult::Cancel)),
                }
            }
        )
    }));

    Ok(Box::new(container))
}
