use sdl::event::Key;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::dialog_button::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::title::Title;
use ui::component::game::GamePrivate;
use ui::component::dialog::DialogResult;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;
use storage::SavedGame;

pub fn new_load_game_dialog(saved_games: &[Option<SavedGame>], messages: &Messages) -> Result<Modal<DialogResult<GamePrivate>>> {
    let rect = Rect::new(250, 90, 300, 420);

    let mut container = Modal::<DialogResult<GamePrivate>>::new();

    container.push(WidgetMapAction::no_action(
        Window::new(rect, BLUE_PATTERN)?
    ));
    container.push(WidgetMapAction::no_action(
        Title {
            text: messages.load_game.to_string(),
            rect: Rect::new(250, 95, 300, 40),
        }
    ));

    for (i, game) in saved_games.iter().enumerate() {
        let label: String = match *game {
            Some(ref g) => g.name.to_string(),
            None => messages.empty.to_string(),
        };

        container.push({
            let game2: Option<SavedGame> = (*game).clone();
            WidgetMapAction::new(
                new_dialog_button(Rect::new(260, 150 + (i as i32) * 30, 280, 25), BLUE_PATTERN, &label, None, ())?,
                move |_| {
                    if let Some(ref game3) = game2 {
                        EventReaction::Action(DialogResult::Ok(game3.game.clone()))
                    } else {
                        EventReaction::NoOp
                    }
                }
            )
        });
    }

    container.push(
        new_dialog_button(Rect::new(360, 470, 80, 25), BLUE_PATTERN, messages.cancel, Some(Key::Escape), DialogResult::Cancel)?
    );

    Ok(container)
}
