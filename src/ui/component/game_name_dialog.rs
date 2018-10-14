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
use ui::component::dialog::DialogResult;
use resources::messages::Messages;

pub fn new_game_name(name: &str, messages: &Messages) -> Result<Modal<DialogResult<String>>> {
    let rect = Rect::new(170, 280, 460, 100);
    let bg = BackgroundPattern::Blue;
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(
        name.to_string()
    ));

    let container = Modal::<DialogResult<String>>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new0(460, 100), bg)
        ))
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
                DialogButton::new(Rect::new(140, 60, 80, 25), bg, messages.ok, Some(Key::Return), ()),
                move |_| {
                    let value: String = state2.borrow().clone();
                    Ok(EventReaction::action(DialogResult::Ok(value)))
                }
            )
        })
        .add(
            DialogButton::new(Rect::new(230, 60, 80, 25), bg, messages.cancel, Some(Key::Escape), DialogResult::Cancel)
        );

    Ok(container)
}
