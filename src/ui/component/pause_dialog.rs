use sdl;
use sdl::video::{Surface};
use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::window::*;
use ui::widget::container::*;
use ui::widget::any_key::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use resources::fonts::*;
use ui::main_loop::main_loop;
use resources::background::{BLUE_PATTERN, GREEN_PATTERN};
use ui::component::background::*;
use locale::get_language;
use storage::Scores;
use util::time::sec_to_str;

fn new_pause_dialog() -> Result<Container<()>> {
    let rect = Rect::new(0, 0, 800, 600);

    let mut container = Container::new(rect, ());

    container.add(Box::new(Background::new()?));
    container.add(Box::new(Window::new(Rect::new(280, 275, 240, 50), GREEN_PATTERN)?));

    container.add(Box::new(Label {
        text: "Paused".to_string(), // i18n msg(L"paused")
        rect: Rect::new(280, 275, 240, 50),
        color: Color::RGB(255, 255, 0),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
    }));

    container.add(Box::new(AnyKey::new(|| Some(Effect::Terminate))));

    Ok(container)
}

pub fn pause(surface: &Surface) -> Result<bool> {
    let pause_dialog = new_pause_dialog()?;
    let quit = main_loop(surface, &pause_dialog)?;
    Ok(quit)
}
