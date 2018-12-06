use sdl2::keyboard::Keycode;
use crate::error::*;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::widget::dialog_button::*;
use crate::ui::component::dialog::dialod_widget;
use crate::resources::messages::Messages;

#[derive(Clone, Copy)]
pub enum FailureChoice {
    StartNew,
    TryAgain,
    Cancel
}

pub fn new_failure_dialog(messages: &Messages) -> Result<Container<FailureChoice>> {
    let bg = Background::RED_PATTERN;

    let container = Container::<FailureChoice>::container(Size::new(360, 140), bg)
        .add(30, 10, WidgetMapAction::no_action(
            Label::title(Size::new(300, 100), messages.loose)
        ))
        .add(30, 100,
            DialogButton::new(Size::new(90, 25), bg, messages.start_new, None, FailureChoice::StartNew)
        )
        .add(130, 100,
            DialogButton::new(Size::new(90, 25), bg, messages.try_again, None, FailureChoice::TryAgain)
        )
        .add(230, 100,
            DialogButton::new(Size::new(90, 25), bg, messages.exit, Some(Keycode::Escape), FailureChoice::Cancel)
        );

    Ok(dialod_widget(None, container))
}
