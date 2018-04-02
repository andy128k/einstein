use std::rc::Rc;
use debug_cell::RefCell;
use sdl::event::Key;
use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::input_field::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;

pub fn new_player_name_dialog(name: &str, messages: &Messages) -> Result<Modal<String>> {
    let rect = Rect::new(170, 280, 460, 100);
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(name.to_string()));

    let container = Modal::<String>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new0(460, 100), BLUE_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(
            Label {
                text: messages.enter_name.to_string(),
                rect: Rect::new(180, 300, 150, 26),
                color: yellow,
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Middle,
            }
        ))
        .add({
            let state2 = state.clone();
            WidgetMapAction::new(
                InputField::new(Rect::new(170, 20, 280, 26), name, 20)?,
                move |name| {
                    *state2.borrow_mut() = name.to_string();
                    EventReaction::Redraw
                }
            )
        })
        .add({
            let state2 = state.clone();
            WidgetMapAction::new(
                new_dialog_button(Rect::new(348, 340, 90, 25), BLUE_PATTERN, messages.ok, Some(Key::Return), ())?,
                move |_| {
                    let result: String = state2.borrow().clone();
                    EventReaction::Action(result)
                }
            )
        });

    Ok(container)
}
