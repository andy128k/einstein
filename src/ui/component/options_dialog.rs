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
use crate::ui::widget::container::Container;
use crate::ui::widget::checkbox::*;
use crate::ui::widget::slider::*;
use crate::ui::component::dialog::*;
use crate::resources::messages::Messages;
use crate::storage::Storage;

#[derive(Clone)]
pub struct Options {
    pub fullscreen: bool,
    pub volume: u32,
    volume_float: f32
}

pub fn new_options_dialog(storage: &Storage, messages: &Messages) -> Result<Container<DialogResult<Options>>> {
    let rect = Rect::new(250, 170, 300, 260);
    let bg = Background::BLUE_PATTERN;

    let state = Rc::new(RefCell::new(Options {
        fullscreen: storage.fullscreen,
        volume: storage.volume,
        volume_float: storage.volume as f32 / 100_f32,
    }));

    let mut container = Container::<DialogResult<Options>>::modal(rect, bg);

    container.push(WidgetMapAction::no_action(
        Label::title(Rect::new(0, 5, 300, 40), messages.options)
    ));
    container.push({
        let state2 = state.clone();
        WidgetMapAction::new(
            Checkbox::new(15, 90, bg, state.borrow().fullscreen),
            move |value| {
                state2.borrow_mut().fullscreen = *value;
                Ok(EventReaction::empty())
            }
        )
    });
    container.push(WidgetMapAction::no_action(
        Label::new(Rect::new(50, 90, 300, 20), messages.fullscreen, Color::RGB(255, 255, 255), HorizontalAlign::Left)
    ));
    container.push(WidgetMapAction::no_action(
        Label::new(Rect::new(15, 160, 300, 20), messages.volume, Color::RGB(255, 255, 255), HorizontalAlign::Left)
    ));
    container.push({
        let state2 = state.clone();
        WidgetMapAction::new(
            Slider::new(Rect::new(110, 162, 160, 16), bg, state.borrow().volume_float),
            move |value| {
                state2.borrow_mut().volume = (*value * 100f32) as u32;
                state2.borrow_mut().volume_float = *value;
                Ok(EventReaction::empty())
            }
        )
    });
    container.push({
        let state2 = state.clone();
        WidgetMapAction::new(
            DialogButton::new(Rect::new(65, 220, 85, 25), bg, messages.ok, Some(Keycode::Return), ()),
            move |_| {
                let s: Options = state2.borrow().clone();
                Ok(EventReaction::action(DialogResult::Ok(s)))
            }
        )
    });
    container.push(
        DialogButton::new(Rect::new(155, 220, 85, 25), bg, messages.cancel,
            Some(Keycode::Escape),
            DialogResult::Cancel
        )
    );

    Ok(container)
}
