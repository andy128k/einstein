use std::rc::Rc;
use debug_cell::RefCell;
use sdl::video::{Surface};
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::dialog::*;
use ui::widget::checkbox::*;
use ui::widget::slider::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::component::dialog::*;
use ui::main_loop::{main_loop, ModalResult};
use resources::background::BLUE_PATTERN;
use resources::messages::{Messages, get_messages};
use storage::Storage;

#[derive(Clone)]
pub struct Options {
    pub fullscreen: bool,
    pub volume: u32,
    volume_float: f32
}

pub fn new_options_dialog(storage: &Storage, messages: &Messages) -> Result<WidgetPtr<ModalResult<DialogResult<Options>>>> {
    let rect = Rect::new(250, 170, 300, 260);

    let state = Rc::new(RefCell::new(Options {
        fullscreen: storage.fullscreen,
        volume: storage.volume,
        volume_float: storage.volume as f32 / 100_f32,
    }));

    let container: Vec<WidgetPtr<ModalResult<DialogResult<Options>>>> = vec![
        Box::new(
            InterceptWidget::default()
        ),
        Box::new(WidgetMapAction::no_action(
            Window::new(rect, BLUE_PATTERN)?
        )),
        Box::new(WidgetMapAction::no_action(
            Title {
                text: messages.options.to_string(),
                rect: Rect::new(250, 175, 300, 40),
            }
        )),
        Box::new({
            let state2 = state.clone();
            WidgetMapAction::new(
                Checkbox::new(Rect::new(265, 260, 20, 20), BLUE_PATTERN, state.borrow().fullscreen)?,
                move |value| {
                    state2.borrow_mut().fullscreen = *value;
                    EventReaction::Redraw
                }
            )
        }),
        Box::new(WidgetMapAction::no_action(
            Label {
                text: messages.fullscreen.to_string(),
                rect: Rect::new(300, 260, 300, 20),
                color: Color::RGB(255, 255, 255),
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Middle,
            }
        )),
        Box::new(WidgetMapAction::no_action(
            Label {
                text: messages.volume.to_string(),
                rect: Rect::new(265, 330, 300, 20),
                color: Color::RGB(255, 255, 255),
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Middle,
            }
        )),
        Box::new({
            let state2 = state.clone();
            WidgetMapAction::new(
                Slider::new(Rect::new(360, 332, 160, 16), BLUE_PATTERN, state.borrow().volume_float)?,
                move |value| {
                    state2.borrow_mut().volume = (*value * 100f32) as u32;
                    state2.borrow_mut().volume_float = *value;
                    EventReaction::Redraw
                }
            )
        }),
        Box::new({
            let state2 = state.clone();
            WidgetMapAction::new(
                new_dialog_button(Rect::new(315, 390, 85, 25), BLUE_PATTERN, messages.ok, Some(Key::Return), ())?,
                move |_| {
                    let s: Options = state2.borrow().clone();
                    EventReaction::Action(ModalResult(DialogResult::Ok(s)))
                }
            )
        }),
        Box::new(new_dialog_button(Rect::new(405, 390, 85, 25), BLUE_PATTERN, messages.cancel,
            Some(Key::Escape),
            ModalResult(DialogResult::Cancel)
        )?)
    ];
    Ok(Box::new(container))
}

pub fn show_options_window(surface: &Surface, storage: &mut Storage) -> Result<bool> {
    let rect = Rect::new(250, 170, 300, 260);

    let container = new_options_dialog(storage, get_messages())?;
    let result = main_loop(&surface, rect, &*container)?;
    match result {
        None => Ok(true), // quit
        Some(DialogResult::Cancel) => Ok(false), // cancelled
        Some(DialogResult::Ok(option_result)) => {
            storage.fullscreen = option_result.fullscreen;
            storage.volume = option_result.volume;

            // screen->setMode(VideoMode(800, 600, 24, option_result.fullscreen));
            // sound->setVolume(option_result.volume);
            Ok(false)
        },
    }
}
