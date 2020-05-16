use crate::cell::RefCell;
use crate::resources::messages::Messages;
use crate::storage::SavedGame;
use crate::ui::common::Size;
use crate::ui::component::dialog::*;
use crate::ui::component::game_name_dialog::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::widget::widget::*;
use sdl2::keyboard::Keycode;
use std::rc::Rc;

pub fn new_save_game_dialog(
    saved_games: &[Option<SavedGame>],
    messages: &'static Messages,
) -> Container<DialogResult<(usize, String)>> {
    let theme = DialogTheme::Blue;

    let mut container = dialog_container(Size::new(300, 420), theme);

    container.push(
        0,
        5,
        Label::title(Size::new(300, 40), messages.save_game).no_action(),
    );

    let ask_name: Rc<RefCell<Option<(usize, String)>>> = Rc::new(RefCell::new(None));

    for (i, game) in saved_games.iter().enumerate() {
        let (label, default_name): (String, String) = match *game {
            Some(ref g) => (g.name.to_string(), g.name.to_string()),
            None => (
                messages.empty.to_string(),
                format!("{} {}", messages.default_game_name, i + 1),
            ),
        };

        container.push(10, 60 + (i as u32) * 30, {
            let ask_name2 = ask_name.clone();
            DialogButton::new(Size::new(280, 25), theme, &label, &[], ()).flat_map_action(
                move |_, _| {
                    *ask_name2.borrow_mut() = Some((i, default_name.clone()));
                    Ok(EventReaction::empty())
                },
            )
        });
    }

    container.push(
        110,
        380,
        DialogButton::new(
            Size::new(80, 25),
            theme,
            messages.close,
            &[Keycode::Escape],
            DialogResult::Cancel,
        ),
    );

    container.push(
        0,
        0,
        cond_dialog(&ask_name, move |&(index, ref name)| {
            new_game_name(name, messages)
                .map_action(move |action| action.map(|name| (index, name.to_string())))
        }),
    );

    dialog_widget(None, container)
}
