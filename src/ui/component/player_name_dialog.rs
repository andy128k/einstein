use std::rc::Rc;
use cell::RefCell;
use sdl::event::Key;
use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::input_field::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use resources::messages::Messages;

pub fn new_player_name_dialog(name: &str, messages: &Messages) -> Result<Modal<String>> {
    let rect = Rect::new(170, 280, 460, 100);
    let bg = BackgroundPattern::Blue;
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(name.to_string()));

    let container = Modal::<String>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new0(460, 100), bg)
        ))
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
                DialogButton::new(Rect::new(178, 60, 90, 25), bg, messages.ok, Some(Key::Return), ()),
                move |_| {
                    let result: String = state2.borrow().clone();
                    Ok(EventReaction::action(result))
                }
            )
        });

    Ok(container)
}
