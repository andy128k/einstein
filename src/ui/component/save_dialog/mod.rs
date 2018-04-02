use std::rc::Rc;
use debug_cell::RefCell;
use sdl::event::Key;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::dialog_button::*;
use ui::widget::dialog::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::title::Title;
use ui::component::game_name_dialog::*;
use ui::component::dialog::DialogResult;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;
use storage::SavedGame;

pub fn new_save_game_dialog(saved_games: &[Option<SavedGame>], messages: &'static Messages) -> Result<Modal<DialogResult<(usize, String)>>> {
    let rect = Rect::new(250, 90, 300, 420);

    let mut container = Modal::<DialogResult<(usize, String)>>::new(rect);

    container.push(WidgetMapAction::no_action(
        Window::new(Rect::new0(300, 420), BLUE_PATTERN)?
    ));

    container.push(WidgetMapAction::no_action(
        Title {
            text: messages.save_game.to_string(),
            rect: Rect::new(0, 5, 300, 40),
        }
    ));

    let ask_name: Rc<RefCell<Option<(usize, String)>>> = Rc::new(RefCell::new(None));

    for (i, game) in saved_games.iter().enumerate() {
        let (label, default_name): (String, String) = match *game {
            Some(ref g) => (g.name.to_string(), g.name.to_string()),
            None => (
                messages.empty.to_string(),
                format!("{} {}", messages.default_game_name, i + 1)
            )
        };

        container.push({
            let ask_name2 = ask_name.clone();
            WidgetMapAction::new(
                new_dialog_button(Rect::new(260, 150 + (i as i32) * 30, 280, 25), BLUE_PATTERN, &label, None, ())?,
                move |_| {
                    *ask_name2.borrow_mut() = Some((i, default_name.clone()));
                    EventReaction::Redraw
                }
            )
        });
    }

    container.push(
        new_dialog_button(Rect::new(360, 470, 80, 25), BLUE_PATTERN, messages.close, Some(Key::Escape), DialogResult::Cancel)?
    );

    container.push({
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
                    DialogResult::Ok(ref name) => EventReaction::Action(DialogResult::Ok((index, name.to_string()))),
                    DialogResult::Cancel => EventReaction::Action(DialogResult::Cancel),
                }
            }
        )
    });

    Ok(container)
}
