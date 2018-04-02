use std::rc::Rc;
use debug_cell::RefCell;
use sdl::event::Key;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::context::{HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::input_field::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::component::dialog::DialogResult;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;

pub fn new_game_name(name: &str, messages: &Messages) -> Result<Modal<DialogResult<String>>> {
    let rect = Rect::new(170, 280, 460, 100);
    let yellow = Color::RGB(255, 255, 0);

    let state = Rc::new(RefCell::new(
        name.to_string()
    ));

    let container = Modal::<DialogResult<String>>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(rect, BLUE_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(
            Label {
                text: messages.enter_game.to_string(),
                rect: Rect::new(180, 300, 150, 26),
                color: yellow,
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Middle,
            }
        ))
        .add({
            let state2 = state.clone();
            WidgetMapAction::new(
                InputField::new(Rect::new(340, 300, 280, 26), name, 20)?,
                move |name| {
                    *state2.borrow_mut() = name.to_string();
                    EventReaction::Redraw
                }
            )
        })
        .add({
            let state2 = state.clone();
            WidgetMapAction::new(
                new_dialog_button(Rect::new(310, 340, 80, 25), BLUE_PATTERN, messages.ok, Some(Key::Return), ())?,
                move |_| {
                    let value: String = state2.borrow().clone();
                    EventReaction::Action(DialogResult::Ok(value))
                }
            )
        })
        .add(
            new_dialog_button(Rect::new(400, 340, 80, 25), BLUE_PATTERN, messages.cancel, Some(Key::Escape), DialogResult::Cancel)?
        );

    Ok(container)
}
