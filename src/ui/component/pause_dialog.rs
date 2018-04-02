use sdl::video::{Surface};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::context::Context;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::window::*;
use ui::widget::dialog::*;
use ui::widget::any_key::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::main_loop::{main_loop, ModalResult};
use resources::messages::{Messages, get_messages};
use resources::background::{GREEN_PATTERN};
use ui::component::background::*;

pub fn new_pause_dialog(messages: &Messages) -> Result<WidgetPtr<ModalResult<()>>> {
    let rect = Rect::new(0, 0, 800, 600);

    let container: Vec<WidgetPtr<ModalResult<()>>> = vec![
        Box::new(
            InterceptWidget::default()
        ),
        Box::new(WidgetMapAction::no_action(
            Background::new()?
        )),
        Box::new(WidgetMapAction::no_action(
            Window::new(Rect::new(280, 275, 240, 50), GREEN_PATTERN)?
        )),
        Box::new(WidgetMapAction::no_action(
            Label {
                text: messages.paused.to_string(),
                rect: Rect::new(280, 275, 240, 50),
                color: Color::RGB(255, 255, 0),
                horizontal_align: HorizontalAlign::Center,
                vertical_align: VerticalAlign::Middle,
            }
        )),
        Box::new(
            AnyKey::new(ModalResult(()))
        ),
    ];

    Ok(Box::new(container))
}

pub fn pause(context: &Context) -> Result<bool> {
    let rect = Rect::new(0, 0, 800, 600);

    let pause_dialog = new_pause_dialog(get_messages())?;
    let quit = main_loop(context, rect, &*pause_dialog)?.is_none();
    Ok(quit)
}
