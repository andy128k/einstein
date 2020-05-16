use crate::resources::messages::Messages;
use crate::resources::rules::get_rules;
use crate::ui::common::Size;
use crate::ui::component::dialog::*;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::Label;
use crate::ui::widget::page_view::*;
use crate::ui::widget::widget::*;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 500;
const CLIENT_WIDTH: u32 = 570;
const CLIENT_HEIGHT: u32 = 390;

pub fn new_help_dialog(messages: &Messages) -> Container<()> {
    let state = PageViewState::new(get_rules());

    let theme = DialogTheme::Blue;

    let container = dialog_container(Size::new(WIDTH, HEIGHT), theme)
        .add(
            150,
            10,
            Label::title(Size::new(300, 40), messages.rules).no_action(),
        )
        .add(
            15,
            50,
            PageView::new(Size::new(CLIENT_WIDTH, CLIENT_HEIGHT), &state).no_action(),
        )
        .add(10, 465, {
            let state2 = state.clone();
            DialogButton::new(Size::new(80, 25), theme, messages.prev, &[], ()).flat_map_action(
                move |_, _| {
                    state2.borrow_mut().prev();
                    Ok(EventReaction::empty())
                },
            )
        })
        .add(100, 465, {
            let state2 = state.clone();
            DialogButton::new(Size::new(80, 25), theme, messages.next, &[], ()).flat_map_action(
                move |_, _| {
                    state2.borrow_mut().next();
                    Ok(EventReaction::empty())
                },
            )
        })
        .add(
            510,
            465,
            DialogButton::new(
                Size::new(80, 25),
                theme,
                messages.close,
                &[Keycode::Escape],
                (),
            ),
        );

    dialog_widget(None, container)
}
