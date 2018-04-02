use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::any_key::*;
use resources::messages::Messages;
use resources::background::{GREEN_PATTERN};
use ui::component::background::*;

pub fn new_pause_dialog(messages: &Messages) -> Result<Modal<()>> {
    let rect = Rect::new(0, 0, 800, 600);

    let container = Modal::<()>::new(rect)
        .add(WidgetMapAction::no_action(
            Background::new()?
        ))
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new(280, 275, 240, 50), GREEN_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(
            Label {
                text: messages.paused.to_string(),
                rect: Rect::new(280, 275, 240, 50),
                color: Color::RGB(255, 255, 0),
                horizontal_align: HorizontalAlign::Center,
                vertical_align: VerticalAlign::Middle,
            }
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
