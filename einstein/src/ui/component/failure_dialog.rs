use sdl2::keyboard::Keycode;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::component::dialog::*;
use crate::resources::messages::Messages;

#[derive(Clone, Copy)]
pub enum FailureChoice {
    StartNew,
    TryAgain,
    Cancel
}

pub fn new_failure_dialog(messages: &Messages) -> Container<FailureChoice> {
    let theme = DialogTheme::Red;

    let container = dialog_container(Size::new(360, 140), theme)
        .add(30, 10,
            Label::title(Size::new(300, 100), messages.loose).no_action()
        )
        .add(30, 100,
            DialogButton::new(Size::new(90, 25), theme, messages.start_new, &[], FailureChoice::StartNew)
        )
        .add(130, 100,
            DialogButton::new(Size::new(90, 25), theme, messages.try_again, &[], FailureChoice::TryAgain)
        )
        .add(230, 100,
            DialogButton::new(Size::new(90, 25), theme, messages.exit, &[Keycode::Escape], FailureChoice::Cancel)
        );

    dialog_widget(None, container)
}
