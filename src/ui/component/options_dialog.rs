use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
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

pub fn new_options_dialog(storage: &Storage, messages: &Messages) -> Container<DialogResult<Options>> {
    let bg = Background::BLUE_PATTERN;

    let state = Rc::new(RefCell::new(Options {
        fullscreen: storage.fullscreen,
        volume: storage.volume,
        volume_float: storage.volume as f32 / 100_f32,
    }));

    let mut container = Container::<DialogResult<Options>>::container(Size::new(300, 260), bg, Border::Raised);

    container.push(0, 5, WidgetMapAction::no_action(
        Label::title(Size::new(300, 40), messages.options)
    ));
    container.push(15, 90, {
        let state2 = state.clone();
        WidgetMapAction::new(
            Checkbox::new(bg, state.borrow().fullscreen),
            move |value, _| {
                state2.borrow_mut().fullscreen = *value;
                Ok(EventReaction::empty())
            }
        )
    });
    container.push(50, 90, WidgetMapAction::no_action(
        Label::new(Size::new(300, 20), messages.fullscreen, Color::RGB(255, 255, 255), HorizontalAlign::Left)
    ));
    container.push(15, 160, WidgetMapAction::no_action(
        Label::new(Size::new(300, 20), messages.volume, Color::RGB(255, 255, 255), HorizontalAlign::Left)
    ));
    container.push(110, 162, {
        let state2 = state.clone();
        WidgetMapAction::new(
            Slider::new(Size::new(160, 16), bg, state.borrow().volume_float),
            move |value, _| {
                state2.borrow_mut().volume = (*value * 100f32) as u32;
                state2.borrow_mut().volume_float = *value;
                Ok(EventReaction::empty())
            }
        )
    });
    container.push(65, 220, {
        let state2 = state.clone();
        WidgetMapAction::new(
            DialogButton::new(Size::new(85, 25), bg, messages.ok, Some(Keycode::Return), ()),
            move |_, _| {
                let s: Options = state2.borrow().clone();
                Ok(EventReaction::action(DialogResult::Ok(s)))
            }
        )
    });
    container.push(155, 220,
        DialogButton::new(Size::new(85, 25), bg, messages.cancel,
            Some(Keycode::Escape),
            DialogResult::Cancel
        )
    );

    dialod_widget(None, container)
}
