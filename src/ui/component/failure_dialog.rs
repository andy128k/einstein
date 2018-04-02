use sdl::event::Key;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::title::Title;
use ui::widget::dialog_button::new_dialog_button2;
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
            Window::new(Rect::new0(360, 140), RED_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(
            Title { rect: Rect::new(30, 10, 300, 100), text: messages.loose.to_string() }
        ))
        .add(
            new_dialog_button2(Rect::new(30, 100, 90, 25), RED_PATTERN, messages.start_new, None, FailureChoice::StartNew)?
        )
        .add(
            new_dialog_button2(Rect::new(130, 100, 90, 25), RED_PATTERN, messages.try_again, None, FailureChoice::TryAgain)?
        )
        .add(
            new_dialog_button2(Rect::new(230, 100, 90, 25), RED_PATTERN, messages.exit, Some(Key::Escape), FailureChoice::Cancel)?
        );

    Ok(container)
}
