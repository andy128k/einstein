use sdl::event::Key;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::title::Title;
use ui::widget::dialog_button::new_dialog_button;
use resources::background::RED_PATTERN;
use resources::messages::Messages;

#[derive(Clone, Copy)]
pub enum FailureChoice {
    StartNew,
    TryAgain,
    Cancel
}

pub fn new_failure_dialog(messages: &Messages) -> Result<Modal<FailureChoice>> {
    let rect = Rect::new(220, 240, 360, 140);

    let container = Modal::<FailureChoice>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(rect, RED_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(
            Title { rect: Rect::new(250, 230, 300, 100), text: messages.loose.to_string() }
        ))
        .add(
            new_dialog_button(Rect::new(250, 340, 90, 25), RED_PATTERN, messages.start_new, None, FailureChoice::StartNew)?
        )
        .add(
            new_dialog_button(Rect::new(350, 340, 90, 25), RED_PATTERN, messages.try_again, None, FailureChoice::TryAgain)?
        )
        .add(
            new_dialog_button(Rect::new(450, 340, 90, 25), RED_PATTERN, messages.exit, Some(Key::Escape), FailureChoice::Cancel)?
        );

    Ok(container)
}
