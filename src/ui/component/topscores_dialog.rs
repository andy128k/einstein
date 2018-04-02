use sdl::event::{Key};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use error::*;
use ui::context::{HorizontalAlign, VerticalAlign};
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::window::*;
use ui::widget::dialog::*;
use ui::widget::title::Title;
use resources::background::BLUE_PATTERN;
use resources::messages::Messages;
use storage::{Scores};
use util::time::sec_to_str;

pub fn create_topscores_dialog(scores: &Scores, messages: &Messages, highlight: Option<usize>) -> Result<WidgetPtr<()>> {
    let rect = Rect::new(240, 125, 320, 350);

    let mut container: Vec<WidgetPtr<()>> = vec![];

    container.push(Box::new(
        InterceptWidget::default()
    ));

    container.push(Box::new(WidgetMapAction::no_action(
        Window::new(rect.clone(), BLUE_PATTERN)?
    )));

    container.push(Box::new(WidgetMapAction::no_action(
        Title {
            text: messages.top_scores.to_string(),
            rect: Rect::new(250, 135, 300, 40),
        }
    )));

    let mut pos = 175;
    for (i, score) in scores.0.iter().enumerate() {
        let color = if highlight == Some(i) {
            Color::RGB(255, 255, 0)
        } else {
            Color::RGB(255, 255, 255)
        };

        container.push(Box::new(WidgetMapAction::no_action(
            Label {
                text: format!("{}.", i + 1),
                rect: Rect::new(250, pos, 30, 25),
                color,
                horizontal_align: HorizontalAlign::Right,
                vertical_align: VerticalAlign::Middle,
            }
        )));

        container.push(Box::new(WidgetMapAction::no_action(
            Label {
                text: score.name.clone(),
                rect: Rect::new(290, pos, 160, 25),
                color,
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Middle,
            }
        )));

        container.push(Box::new(WidgetMapAction::no_action(
            Label {
                text: sec_to_str(score.score),
                rect: Rect::new(460, pos, 80, 25),
                color,
                horizontal_align: HorizontalAlign::Right,
                vertical_align: VerticalAlign::Middle,
            }
        )));

        pos += 25;
    }

    container.push(Box::new(
        new_dialog_button(Rect::new(355, 435, 90, 25), BLUE_PATTERN, messages.ok, Some(Key::Escape), ())?
    ));

    Ok(Box::new(container))
}
