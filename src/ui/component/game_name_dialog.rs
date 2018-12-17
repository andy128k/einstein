use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::error::*;
use crate::ui::context::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::label::*;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::input_field::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::{DialogResult, dialod_widget};
use crate::resources::messages::Messages;

pub fn new_game_name(name: &str, messages: &Messages) -> Result<Container<DialogResult<String>>> {
    let bg = Background::BLUE_PATTERN;
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(
        name.to_string()
    ));

    let container = Container::<DialogResult<String>>::container(Size::new(460, 100), bg, Border::Raised)
        .add(10, 20, WidgetMapAction::no_action(
            Label::new(Size::new(150, 26), messages.enter_game, yellow, HorizontalAlign::Left)
        ))
        .add(170, 20, {
            let state2 = state.clone();
            WidgetMapAction::new(
                InputField::new(Size::new(280, 26), name, 20)?,
                move |name, _, _| {
                    *state2.borrow_mut() = name.to_string();
                    Ok(EventReaction::empty())
                }
            )
        })
        .add(140, 60, {
            let state2 = state.clone();
            WidgetMapAction::new(
                DialogButton::new(Size::new(80, 25), bg, messages.ok, Some(Keycode::Return), ()),
                move |_, _, _| {
                    let value: String = state2.borrow().clone();
                    Ok(EventReaction::action(DialogResult::Ok(value)))
                }
            )
        })
        .add(230, 60,
            DialogButton::new(Size::new(80, 25), bg, messages.cancel, Some(Keycode::Escape), DialogResult::Cancel)
        );

    Ok(dialod_widget(None, container))
}
