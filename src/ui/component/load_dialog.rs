use sdl::event::Key;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::dialog_button::*;
use ui::widget::container::Container;
use ui::widget::label::Label;
use ui::component::game::GamePrivate;
use ui::component::dialog::DialogResult;
use resources::messages::Messages;
use storage::SavedGame;

pub fn new_load_game_dialog(saved_games: &[Option<SavedGame>], messages: &Messages) -> Result<Container<DialogResult<GamePrivate>>> {
    let rect = Rect::new(250, 90, 300, 420);
    let bg = BackgroundPattern::Blue;

    let mut container = Container::<DialogResult<GamePrivate>>::modal(rect, bg);

    container.push(WidgetMapAction::no_action(
        Label::title(Rect::new(0, 5, 300, 40), messages.load_game)
    ));

    for (i, game) in saved_games.iter().enumerate() {
        let label: String = match *game {
            Some(ref g) => g.name.to_string(),
            None => messages.empty.to_string(),
        };

        container.push({
            let game2: Option<SavedGame> = (*game).clone();
            WidgetMapAction::new(
                DialogButton::new(Rect::new(10, 60 + (i as i32) * 30, 280, 25), bg, &label, None, ()),
                move |_| {
                    if let Some(ref game3) = game2 {
                        Ok(EventReaction::action(DialogResult::Ok(game3.game.clone())))
                    } else {
                        Ok(EventReaction::empty())
                    }
                }
            )
        });
    }

    container.push(
        DialogButton::new(Rect::new(110, 380, 80, 25), bg, messages.cancel, Some(Key::Escape), DialogResult::Cancel)
    );

    Ok(container)
}
