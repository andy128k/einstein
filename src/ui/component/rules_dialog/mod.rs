use std::rc::Rc;
use cell::RefCell;
use sdl2::keyboard::Keycode;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::dialog_button::*;
use ui::widget::label::Label;
use ui::widget::page_view::*;
use ui::widget::container::Container;
use resources::rules::{get_rules, TextItem};
use resources::messages::Messages;

const WIDTH: u16 = 600;
const HEIGHT: u16 = 500;
const CLIENT_WIDTH: u16 = 570;
const CLIENT_HEIGHT: u16 = 390;

struct DescriptionPrivate {
    page_view_state: Rc<RefCell<PageViewState>>,
}

impl DescriptionPrivate {
    fn new(messages: &Messages, text: &'static [TextItem]) -> Result<Container<()>> {
        let page_view_state = PageViewState::new(text);

        let rect = Rect::new(100, 50, WIDTH as u32, HEIGHT as u32);
        let bg = BackgroundPattern::Blue;

        let state = Rc::new(RefCell::new(DescriptionPrivate {
            page_view_state: page_view_state.clone(),
        }));

        let container = Container::<()>::modal(rect, bg)
            .add(WidgetMapAction::no_action(
                Label::title(Rect::new(150, 10, 300, 40), messages.rules)
            ))
            .add(WidgetMapAction::no_action(
                PageView::new(Rect::new(15, 50, CLIENT_WIDTH as u32, CLIENT_HEIGHT as u32), &page_view_state)
            ))
            .add({
                let state2 = state.clone();
                WidgetMapAction::new(
                    DialogButton::new(Rect::new(10, 465, 80, 25), bg, messages.prev, None, ()),
                    move |_| {
                        state2.borrow_mut().prev();
                        Ok(EventReaction::empty())
                    }
                )
            })
            .add({
                let state2 = state.clone();
                WidgetMapAction::new(
                    DialogButton::new(Rect::new(100, 465, 80, 25), bg, messages.next, None, ()),
                    move |_| {
                        state2.borrow_mut().next();
                        Ok(EventReaction::empty())
                    }
                )
            })
            .add(
                DialogButton::new(Rect::new(510, 465, 80, 25), bg, messages.close, Some(Keycode::Escape), ())
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
    DescriptionPrivate::new(messages, get_rules())
}
