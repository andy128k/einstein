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
use crate::ui::component::dialog::DialogResult;
use crate::resources::messages::Messages;

pub fn new_game_name(name: &str, messages: &Messages) -> Result<Container<DialogResult<String>>> {
    let rect = Rect::new(170, 280, 460, 100);
    let bg = Background::BLUE_PATTERN;
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(
        name.to_string()
    ));

    let container = Container::<DialogResult<String>>::modal(rect, bg)
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(10, 20, 150, 26), messages.enter_game, yellow, HorizontalAlign::Left)
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
                DialogButton::new(Rect::new(140, 60, 80, 25), bg, messages.ok, Some(Keycode::Return), ()),
                move |_| {
                    let value: String = state2.borrow().clone();
                    Ok(EventReaction::action(DialogResult::Ok(value)))
                }
            )
        })
        .add(
            DialogButton::new(Rect::new(230, 60, 80, 25), bg, messages.cancel, Some(Keycode::Escape), DialogResult::Cancel)
        );

    Ok(container)
}
