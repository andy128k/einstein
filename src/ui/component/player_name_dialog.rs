use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::context::{Rect, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::label::*;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::input_field::*;
use crate::ui::widget::container::Container;
use crate::resources::messages::Messages;

pub fn new_player_name_dialog(name: &str, messages: &Messages) -> Result<Container<String>> {
    let rect = Rect::new(170, 280, 460, 100);
    let bg = Background::BLUE_PATTERN;
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(name.to_string()));

    let container = Container::<String>::modal(rect, bg)
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(10, 20, 150, 26), messages.enter_name, yellow, HorizontalAlign::Left)
        ))
        .add({
            let state2 = state.clone();
            WidgetMapAction::new(
                InputField::new(Rect::new(170, 20, 280, 26), name, 20)?,
                move |name| {
                    *state2.borrow_mut() = name.to_string();
                    Ok(EventReaction::empty())
                }
            )
        })
        .add({
            let state2 = state.clone();
            WidgetMapAction::new(
                DialogButton::new(Rect::new(178, 60, 90, 25), bg, messages.ok, Some(Keycode::Return), ()),
                move |_| {
                    let result: String = state2.borrow().clone();
                    Ok(EventReaction::action(result))
                }
            )
        });

    Ok(container)
}
