use std::rc::Rc;
use debug_cell::RefCell;
use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::modal::Modal;
use ui::widget::checkbox::*;
use ui::widget::slider::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::component::dialog::*;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;
use storage::Storage;

#[derive(Clone)]
pub struct Options {
    pub fullscreen: bool,
    pub volume: u32,
    volume_float: f32
}

pub fn new_options_dialog(storage: &Storage, messages: &Messages) -> Result<Modal<DialogResult<Options>>> {
    let rect = Rect::new(250, 170, 300, 260);

    let state = Rc::new(RefCell::new(Options {
        fullscreen: storage.fullscreen,
        volume: storage.volume,
        volume_float: storage.volume as f32 / 100_f32,
    }));

    let mut container = Modal::<DialogResult<Options>>::new(rect);

    container.push(WidgetMapAction::no_action(
        Window::new(rect, BLUE_PATTERN)?
    ));
    container.push(WidgetMapAction::no_action(
        Title {
            text: messages.options.to_string(),
            rect: Rect::new(0, 5, 300, 40),
        }
    ));
    container.push({
        let state2 = state.clone();
        WidgetMapAction::new(
            Checkbox::new(15, 90, BLUE_PATTERN, state.borrow().fullscreen)?,
            move |value| {
                state2.borrow_mut().fullscreen = *value;
                EventReaction::Redraw
            }
        )
    });
    container.push(WidgetMapAction::no_action(
        Label {
            text: messages.fullscreen.to_string(),
            rect: Rect::new(300, 260, 300, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
        }
    ));
    container.push(WidgetMapAction::no_action(
        Label {
            text: messages.volume.to_string(),
            rect: Rect::new(265, 330, 300, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
        }
    ));
    container.push({
        let state2 = state.clone();
        WidgetMapAction::new(
            Slider::new(Rect::new(360, 332, 160, 16), BLUE_PATTERN, state.borrow().volume_float)?,
            move |value| {
                state2.borrow_mut().volume = (*value * 100f32) as u32;
                state2.borrow_mut().volume_float = *value;
                EventReaction::Redraw
            }
        )
    });
    container.push({
        let state2 = state.clone();
        WidgetMapAction::new(
            new_dialog_button(Rect::new(315, 390, 85, 25), BLUE_PATTERN, messages.ok, Some(Key::Return), ())?,
            move |_| {
                let s: Options = state2.borrow().clone();
                EventReaction::Action(DialogResult::Ok(s))
            }
        )
    });
    container.push(
        new_dialog_button(Rect::new(405, 390, 85, 25), BLUE_PATTERN, messages.cancel,
            Some(Key::Escape),
            DialogResult::Cancel
        )?
    );

    Ok(container)
}
