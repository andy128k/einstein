use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::label::*;
use ui::widget::container::Container;
use ui::widget::any_key::*;
use resources::messages::Messages;

pub fn new_pause_dialog(messages: &Messages) -> Result<Container<()>> {
    let rect = Rect::new(280, 275, 240, 50);

    let container = Container::<()>::modal(rect, BackgroundPattern::Green)
        .add(WidgetMapAction::no_action(
            Label::new(rect.relative(), messages.paused, Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
