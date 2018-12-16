use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use crate::error::*;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::conditional::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::component::game_name_dialog::*;
use crate::ui::component::dialog::{DialogResult, dialod_widget};
use crate::resources::messages::Messages;
use crate::storage::SavedGame;

pub fn new_save_game_dialog(saved_games: &[Option<SavedGame>], messages: &'static Messages) -> Result<Container<DialogResult<(usize, String)>>> {
    let bg = Background::BLUE_PATTERN;

    let mut container = Container::<DialogResult<(usize, String)>>::container(Size::new(300, 420), bg);

    container.push(0, 5, WidgetMapAction::no_action(
        Label::title(Size::new(300, 40), messages.save_game)
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

        container.push(10, 60 + (i as u32) * 30, {
            let ask_name2 = ask_name.clone();
            WidgetMapAction::new(
                DialogButton::new(Size::new(280, 25), bg, &label, None, ()),
                move |_, _, _| {
                    *ask_name2.borrow_mut() = Some((i, default_name.clone()));
                    Ok(EventReaction::empty())
                }
            )
        });
    }

    container.push(110, 380,
        DialogButton::new(Size::new(80, 25), bg, messages.close, Some(Keycode::Escape), DialogResult::Cancel)
    );

    container.push(0, 0, {
        let ask_name2 = ask_name.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                ask_name.clone(),
                move |&(_index, ref name)| new_game_name(name, messages)
            ),
            move |result, _, _| {
                let index = ask_name2.borrow().as_ref().map(|p| p.0).unwrap();
                *ask_name2.borrow_mut() = None;
                match *result {
                    DialogResult::Ok(ref name) => Ok(EventReaction::action(DialogResult::Ok((index, name.to_string())))),
                    DialogResult::Cancel => Ok(EventReaction::action(DialogResult::Cancel)),
                }
            }
        )
    });

    Ok(dialod_widget(None, container))
}
