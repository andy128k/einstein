use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::context::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::label::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::any_key::*;
use crate::ui::component::dialog::dialod_widget;
use crate::resources::messages::Messages;

pub fn new_pause_dialog(messages: &Messages) -> Result<Container<()>> {
    let size = Size::new(240, 50);

    let container = Container::<()>::container(size, Background::GREEN_PATTERN)
        .add(0, 0, WidgetMapAction::no_action(
            Label::new(size, messages.paused, Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(0, 0,
            AnyKey::new(())
        );

    Ok(dialod_widget(None, container))
}
