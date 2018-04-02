use std::rc::Rc;
use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Key};
use sdl2::rect::Rect;
use error::*;
use ui::context::Context;
use ui::widget::widget::*;
use ui::widget::window::*;
use ui::widget::dialog::*;
use ui::widget::title::Title;
use ui::widget::dialog_button::new_dialog_button;
use ui::main_loop::{main_loop, ModalResult};
use resources::background::RED_PATTERN;
use resources::messages::{Messages, get_messages};

#[derive(Clone, Copy)]
pub enum FailureChoice {
    StartNew,
    TryAgain,
    Cancel
}

pub fn new_failure_dialog(messages: &Messages) -> Result<WidgetPtr<ModalResult<FailureChoice>>> {
    let rect = Rect::new(220, 240, 360, 140);

    let container: Vec<WidgetPtr<ModalResult<FailureChoice>>> = vec![
        Box::new(
            InterceptWidget::default()
        ),
        Box::new(WidgetMapAction::no_action(
            Window::new(rect, RED_PATTERN)?
        )),
        Box::new(WidgetMapAction::no_action(
            Title { rect: Rect::new(250, 230, 300, 100), text: messages.loose.to_string() }
        )),
        Box::new(
            new_dialog_button(Rect::new(250, 340, 90, 25), RED_PATTERN, messages.start_new, None, ModalResult(FailureChoice::StartNew))?
        ),
        Box::new(
            new_dialog_button(Rect::new(350, 340, 90, 25), RED_PATTERN, messages.try_again, None, ModalResult(FailureChoice::TryAgain))?
        ),
        Box::new(
            new_dialog_button(Rect::new(450, 340, 90, 25), RED_PATTERN, messages.exit, Some(Key::Escape), ModalResult(FailureChoice::Cancel))?
        ),
    ];

    Ok(Box::new(container))
}

pub fn show_failure_dialog(context: &Context) -> Result<Option<FailureChoice>> {
    let rect = Rect::new(220, 240, 360, 140);
    let dlg = new_failure_dialog(get_messages())?;
    main_loop(context, rect, &*dlg)
}
