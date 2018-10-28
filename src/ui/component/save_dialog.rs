use std::rc::Rc;
use cell::RefCell;
use sdl::event::Key;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::dialog_button::*;
use ui::widget::conditional::*;
use ui::widget::container::Container;
use ui::widget::label::Label;
use ui::component::game_name_dialog::*;
use ui::component::dialog::DialogResult;
use resources::messages::Messages;
use storage::SavedGame;

pub fn new_save_game_dialog(saved_games: &[Option<SavedGame>], messages: &'static Messages) -> Result<Container<DialogResult<(usize, String)>>> {
    let rect = Rect::new(250, 90, 300, 420);
    let bg = BackgroundPattern::Blue;

    let mut container = Container::<DialogResult<(usize, String)>>::modal(rect, bg);

    container.push(WidgetMapAction::no_action(
        Label::title(Rect::new(0, 5, 300, 40), messages.save_game)
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
                DialogButton::new(Rect::new(10, 60 + (i as i32) * 30, 280, 25), bg, &label, None, ()),
                move |_| {
                    *ask_name2.borrow_mut() = Some((i, default_name.clone()));
                    Ok(EventReaction::empty())
                }
            )
        });
    }

    container.push(
        DialogButton::new(Rect::new(110, 380, 80, 25), bg, messages.close, Some(Key::Escape), DialogResult::Cancel)
    );

    container.push({
        let ask_name2 = ask_name.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                ask_name.clone(),
                move |&(_index, ref name)| new_game_name(name, messages)
            ),
            move |result| {
                let index = ask_name2.borrow().as_ref().map(|p| p.0).unwrap();
                *ask_name2.borrow_mut() = None;
                match *result {
                    DialogResult::Ok(ref name) => Ok(EventReaction::action(DialogResult::Ok((index, name.to_string())))),
                    DialogResult::Cancel => Ok(EventReaction::action(DialogResult::Cancel)),
                }
            }
        )
    });

    Ok(container)
}
