use sdl::video::{Surface};
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::context::Context;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::dialog::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::main_loop::{ModalResult, main_loop};
use resources::background::BLUE_PATTERN;
use resources::messages::{Messages, get_messages};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn create_about_dialog(messages: &Messages) -> Result<WidgetPtr<ModalResult<()>>> {
    let rect = Rect::new(220, 160, 360, 280);
    let container: Vec<WidgetPtr<ModalResult<()>>> = vec![
        Box::new(
            InterceptWidget::default()
        ),
        Box::new(WidgetMapAction::no_action(
            Window::new(rect.clone(), BLUE_PATTERN)?
        )),
        Box::new(WidgetMapAction::no_action(Title {
            text: messages.about.to_string(),
            rect: Rect::new(250, 165, 300, 40),
        })),
        Box::new(WidgetMapAction::no_action(Label {
            text: messages.einstein_puzzle.to_string(),
            rect: Rect::new(220, 240, 360, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        })),
        Box::new(WidgetMapAction::no_action(Label {
            text: format!("{}{}", messages.version, VERSION),
            rect: Rect::new(220, 260, 360, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        })),
        Box::new(WidgetMapAction::no_action(Label {
            text: messages.copyright.to_string(),
            rect: Rect::new(220, 280, 360, 20),
            color: Color::RGB(255, 255, 255),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        })),
        Box::new(WidgetMapAction::no_action(Label {
            text: "http://games.flowix.com".to_string(),
            rect: Rect::new(220, 330, 360, 20),
            color: Color::RGB(255, 255, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        })),
        Box::new(new_dialog_button(Rect::new(360, 400, 80, 25), BLUE_PATTERN, messages.ok,
            Some(Key::Escape), // Return also
            ModalResult(())
        )?),
    ];
    Ok(Box::new(container))
}

pub fn show_about(context: &Context) -> Result<bool> {
    let rect = Rect::new(220, 160, 360, 280);
    let about = create_about_dialog(get_messages())?;
    let quit = main_loop(context, rect, &*about)?.is_none();
    Ok(quit)
}
