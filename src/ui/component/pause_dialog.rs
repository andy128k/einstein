use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::context::{Rect, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::label::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::any_key::*;
use crate::resources::messages::Messages;

pub fn new_pause_dialog(messages: &Messages) -> Result<Container<()>> {
    let rect = Rect::new(280, 275, 240, 50);

    let container = Container::<()>::modal(rect, Background::GREEN_PATTERN)
        .add(WidgetMapAction::no_action(
            Label::new(rect.relative(), messages.paused, Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(
            AnyKey::new(())
        );

    Ok(container)
}
