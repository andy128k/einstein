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
use storage::{Scores};
use util::time::sec_to_str;

pub fn create_topscores_dialog(scores: &Scores, messages: &Messages, highlight: Option<usize>) -> Result<Modal<()>> {
    let rect = Rect::new(240, 125, 320, 350);

    let mut container = Modal::<()>::new(rect);

    container.push(WidgetMapAction::no_action(
        Window::new(Rect::new0(320, 350), BLUE_PATTERN)?
    ));

    container.push(WidgetMapAction::no_action(
        Title {
            text: messages.top_scores.to_string(),
            rect: Rect::new(10, 10, 300, 40),
        }
    ));

    let mut pos = 50;
    for (i, score) in scores.0.iter().enumerate() {
        let color = if highlight == Some(i) {
            Color::RGB(255, 255, 0)
        } else {
            Color::RGB(255, 255, 255)
        };

        container.push(WidgetMapAction::no_action(
            Label::new(Rect::new(10, pos, 30, 25), &format!("{}.", i + 1), color, HorizontalAlign::Right)
        ));
        container.push(WidgetMapAction::no_action(
            Label::new(Rect::new(50, pos, 160, 25), &score.name, color, HorizontalAlign::Left)
        ));
        container.push(WidgetMapAction::no_action(
            Label::new(Rect::new(220, pos, 80, 25), &sec_to_str(score.score), color, HorizontalAlign::Right)
        ));

        pos += 25;
    }

    container.push(
        new_dialog_button(Rect::new(355, 435, 90, 25), BLUE_PATTERN, messages.ok, Some(Key::Escape), ())?
    );

    Ok(container)
}
