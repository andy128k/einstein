use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::label::*;
use crate::ui::widget::input_field::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::*;
use crate::resources::messages::Messages;

pub fn new_player_name_dialog(name: &str, messages: &Messages) -> Container<String> {
    let theme = DialogTheme::Blue;
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(name.to_string()));

    let container = dialog_container(Size::new(460, 100), theme)
        .add(10, 20, WidgetMapAction::no_action(
            Label::new(Size::new(150, 26), messages.enter_name, yellow, HorizontalAlign::Left)
        ))
        .add(170, 20, {
            let state2 = state.clone();
            WidgetMapAction::new(
                InputField::new(Size::new(280, 26), name, 20),
                move |name, _| {
                    *state2.borrow_mut() = name.to_string();
                    Ok(EventReaction::empty())
                }
            )
        })
        .add(178, 60, {
            let state2 = state.clone();
            WidgetMapAction::new(
                DialogButton::new(Size::new(90, 25), theme, messages.ok, &[Keycode::Return], ()),
                move |_, _| {
                    let result: String = state2.borrow().clone();
                    Ok(EventReaction::action(result))
                }
            )
        });

    dialog_widget(None, container)
}