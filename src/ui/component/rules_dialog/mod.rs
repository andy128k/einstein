use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use crate::error::*;
use crate::ui::context::{Rect, Size};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::label::Label;
use crate::ui::widget::page_view::*;
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::dialod_widget;
use crate::resources::rules::{get_rules, TextItem};
use crate::resources::messages::Messages;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 500;
const CLIENT_WIDTH: u32 = 570;
const CLIENT_HEIGHT: u32 = 390;

struct DescriptionPrivate {
    page_view_state: Rc<RefCell<PageViewState>>,
}

impl DescriptionPrivate {
    fn new(messages: &Messages, text: &'static [TextItem]) -> Result<Container<()>> {
        let page_view_state = PageViewState::new(text);

        let bg = Background::BLUE_PATTERN;

        let state = Rc::new(RefCell::new(DescriptionPrivate {
            page_view_state: page_view_state.clone(),
        }));

        let container = Container::<()>::container(Size::new(WIDTH, HEIGHT), bg)
            .add(150, 10, WidgetMapAction::no_action(
                Label::title(Size::new(300, 40), messages.rules)
            ))
            .add(15, 50, WidgetMapAction::no_action(
                PageView::new(Size::new(CLIENT_WIDTH, CLIENT_HEIGHT), &page_view_state)
            ))
            .add(10, 465, {
                let state2 = state.clone();
                WidgetMapAction::new(
                    DialogButton::new(Size::new(80, 25), bg, messages.prev, None, ()),
                    move |_| {
                        state2.borrow_mut().prev();
                        Ok(EventReaction::empty())
                    }
                )
            })
            .add(100, 465, {
                let state2 = state.clone();
                WidgetMapAction::new(
                    DialogButton::new(Size::new(80, 25), bg, messages.next, None, ()),
                    move |_| {
                        state2.borrow_mut().next();
                        Ok(EventReaction::empty())
                    }
                )
            })
            .add(510, 465,
                DialogButton::new(Size::new(80, 25), bg, messages.close, Some(Keycode::Escape), ())
            );

        Ok(container)
    }

    fn prev(&mut self) {
        self.page_view_state.borrow_mut().prev();
    }

    fn next(&mut self) {
        self.page_view_state.borrow_mut().next();
    }
}

pub fn new_help_dialog(messages: &Messages) -> Result<Container<()>> {
    let dlg = DescriptionPrivate::new(messages, get_rules())?;
    Ok(dialod_widget(None, dlg))
}
