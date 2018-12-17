use sdl2::keyboard::Keycode;
use crate::error::*;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::component::game::GamePrivate;
use crate::ui::component::dialog::{DialogResult, dialod_widget};
use crate::resources::messages::Messages;
use crate::storage::SavedGame;

pub fn new_load_game_dialog(saved_games: &[Option<SavedGame>], messages: &Messages) -> Result<Container<DialogResult<GamePrivate>>> {
    let bg = Background::BLUE_PATTERN;

    let mut container = Container::<DialogResult<GamePrivate>>::container(Size::new(300, 420), bg, Border::Raised);

    container.push(0, 5, WidgetMapAction::no_action(
        Label::title(Size::new(300, 40), messages.load_game)
    ));

    for (i, game) in saved_games.iter().enumerate() {
        let label: String = match *game {
            Some(ref g) => g.name.to_string(),
            None => messages.empty.to_string(),
        };

        container.push(10, 60 + (i as u32) * 30, {
            let game2: Option<SavedGame> = (*game).clone();
            WidgetMapAction::new(
                DialogButton::new(Size::new(280, 25), bg, &label, None, ()),
                move |_, _, _| {
                    if let Some(ref game3) = game2 {
                        Ok(EventReaction::action(DialogResult::Ok(game3.game.clone())))
                    } else {
                        Ok(EventReaction::empty())
                    }
                }
            )
        });
    }

    container.push(110, 380,
        DialogButton::new(Size::new(80, 25), bg, messages.cancel, Some(Keycode::Escape), DialogResult::Cancel)
    );

    Ok(dialod_widget(None, container))
}
