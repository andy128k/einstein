use sdl::event::Key;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::label::Label;
use ui::widget::dialog_button::*;
use resources::messages::Messages;

#[derive(Clone, Copy)]
pub enum FailureChoice {
    StartNew,
    TryAgain,
    Cancel
}

pub fn new_failure_dialog(messages: &Messages) -> Result<Modal<FailureChoice>> {
    let rect = Rect::new(220, 240, 360, 140);
    let bg = BackgroundPattern::Red;

    let container = Modal::<FailureChoice>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new0(360, 140), bg)
        ))
        .add(WidgetMapAction::no_action(
            Label::title(Rect::new(30, 10, 300, 100), messages.loose)
        ))
        .add(
            DialogButton::new(Rect::new(30, 100, 90, 25), bg, messages.start_new, None, FailureChoice::StartNew)
        )
        .add(
            DialogButton::new(Rect::new(130, 100, 90, 25), bg, messages.try_again, None, FailureChoice::TryAgain)
        )
        .add(
            DialogButton::new(Rect::new(230, 100, 90, 25), bg, messages.exit, Some(Key::Escape), FailureChoice::Cancel)
        );

    Ok(container)
}
