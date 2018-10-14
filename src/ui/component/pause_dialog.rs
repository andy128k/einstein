use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::label::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::any_key::*;
use resources::messages::Messages;
use ui::component::background::*;

pub fn new_pause_dialog(messages: &Messages) -> Result<Modal<()>> {
    let rect = Rect::new(0, 0, 800, 600);

    let container = Modal::<()>::new(rect)
        .add(WidgetMapAction::no_action(
            Background::new()?
        ))
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new(280, 275, 240, 50), BackgroundPattern::Green)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(280, 275, 240, 50), messages.paused, Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
