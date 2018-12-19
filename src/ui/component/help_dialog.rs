use sdl2::keyboard::Keycode;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::label::Label;
use crate::ui::widget::page_view::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::dialod_widget;
use crate::resources::rules::get_rules;
use crate::resources::messages::Messages;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 500;
const CLIENT_WIDTH: u32 = 570;
const CLIENT_HEIGHT: u32 = 390;

pub fn new_help_dialog(messages: &Messages) -> Container<()> {
    let state = PageViewState::new(get_rules());

    let bg = Background::BLUE_PATTERN;

    let container = Container::<()>::container(Size::new(WIDTH, HEIGHT), bg, Border::Raised)
        .add(150, 10, WidgetMapAction::no_action(
            Label::title(Size::new(300, 40), messages.rules)
        ))
        .add(15, 50, WidgetMapAction::no_action(
            PageView::new(Size::new(CLIENT_WIDTH, CLIENT_HEIGHT), &state)
        ))
        .add(10, 465, {
            let state2 = state.clone();
            WidgetMapAction::new(
                DialogButton::new(Size::new(80, 25), bg, messages.prev, &[], ()),
                move |_, _| {
                    state2.borrow_mut().prev();
                    Ok(EventReaction::empty())
                }
            )
        })
        .add(100, 465, {
            let state2 = state.clone();
            WidgetMapAction::new(
                DialogButton::new(Size::new(80, 25), bg, messages.next, &[], ()),
                move |_, _| {
                    state2.borrow_mut().next();
                    Ok(EventReaction::empty())
                }
            )
        })
        .add(510, 465,
            DialogButton::new(Size::new(80, 25), bg, messages.close, &[Keycode::Escape], ())
        );

    dialod_widget(None, container)
}
