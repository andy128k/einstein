use crate::ui::widget::widget::*;
use crate::ui::widget::label::*;
use crate::ui::widget::any_key::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::*;
use crate::ui::common::{Size, HorizontalAlign};

pub fn create_message_dialog(theme: DialogTheme, message: &str) -> Container<()> {
    let size = Size::new(500, 400);

    let container = dialog_container(size, theme)
        .add(0, 0, WidgetMapAction::no_action(
            Label::new(size, message, theme.text_color(), HorizontalAlign::Center)
        ))
        .add(0, 0,
            AnyKey::new(())
        );

    dialod_widget(None, container)
}
