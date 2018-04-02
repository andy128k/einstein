use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign, VerticalAlign};
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
    let container = Modal::<()>::new(rect)
        .add(WidgetMapAction::no_action(
            Window::new(Rect::new0(360, 280), BLUE_PATTERN)?
        ))
        .add(WidgetMapAction::no_action(Title {
            text: messages.about.to_string(),
            rect: Rect::new(30, 5, 300, 40),
        }))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 80, 360, 20), messages.einstein_puzzle, Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 100, 360, 20), &format!("{}{}", messages.version, VERSION), Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 120, 360, 20), messages.copyright, Color::RGB(255, 255, 255), HorizontalAlign::Center)
        ))
        .add(WidgetMapAction::no_action(
            Label::new(Rect::new(0, 170, 360, 20),"http://games.flowix.com", Color::RGB(255, 255, 0), HorizontalAlign::Center)
        ))
        .add(
            new_dialog_button(Rect::new(360, 400, 80, 25), BLUE_PATTERN, messages.ok,
                Some(Key::Escape), // Return also
                ()
            )?
        );
    Ok(container)
}
