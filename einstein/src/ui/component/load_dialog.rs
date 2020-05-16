use crate::resources::messages::Messages;
use crate::storage::SavedGame;
use crate::ui::common::Size;
use crate::ui::component::dialog::*;
use crate::ui::component::game::GamePrivate;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::widget::widget::*;
use sdl2::keyboard::Keycode;

pub fn new_load_game_dialog(
    saved_games: &[Option<SavedGame>],
    messages: &Messages,
) -> Container<DialogResult<GamePrivate>> {
    let theme = DialogTheme::Blue;

    let mut container = dialog_container(Size::new(300, 420), theme);

    container.push(
        0,
        5,
        Label::title(Size::new(300, 40), messages.load_game).no_action(),
    );

    for (i, game) in saved_games.iter().enumerate() {
        let label: String = match *game {
            Some(ref g) => g.name.to_string(),
            None => messages.empty.to_string(),
        };

        container.push(10, 60 + (i as u32) * 30, {
            let game2: Option<SavedGame> = (*game).clone();
            DialogButton::new(Size::new(280, 25), theme, &label, &[], ()).flat_map_action(
                move |_, _| {
                    if let Some(ref game3) = game2 {
                        Ok(EventReaction::action(DialogResult::Ok(game3.game.clone())))
                    } else {
                        Ok(EventReaction::empty())
                    }
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
            messages.cancel,
            &[Keycode::Escape],
            DialogResult::Cancel,
        ),
    );

    dialog_widget(None, container)
}
