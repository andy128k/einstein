use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::context::{HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::window::*;
use ui::widget::modal::Modal;
use ui::widget::title::Title;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn create_about_dialog(messages: &Messages) -> Result<Modal<()>> {
    let rect = Rect::new(220, 160, 360, 280);
    let container = Modal::<()>::new()
        .add(WidgetMapAction::no_action(
            Window::new(rect.clone(), BLUE_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(Title {
            text: messages.about.to_string(),
            rect: Rect::new(250, 165, 300, 40),
        }))
        .add(WidgetMapAction::no_action(Label {
            text: messages.einstein_puzzle.to_string(),
            rect: Rect::new(220, 240, 360, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        }))
        .add(WidgetMapAction::no_action(Label {
            text: format!("{}{}", messages.version, VERSION),
            rect: Rect::new(220, 260, 360, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        }))
        .add(WidgetMapAction::no_action(Label {
            text: messages.copyright.to_string(),
            rect: Rect::new(220, 280, 360, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        }))
        .add(WidgetMapAction::no_action(Label {
            text: "http://games.flowix.com".to_string(),
            rect: Rect::new(220, 330, 360, 20),
            color: Color::RGB(255, 255, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        }))
        .add(
            new_dialog_button(Rect::new(360, 400, 80, 25), BLUE_PATTERN, messages.ok,
                Some(Key::Escape), // Return also
                ()
            )?
        );
    Ok(container)
}
