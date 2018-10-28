use sdl::event::{Key};
use sdl2::pixels::Color;
use error::*;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::label::*;
use ui::widget::dialog_button::*;
use ui::widget::container::Container;
use resources::messages::Messages;
use storage::{Scores};
use util::time::sec_to_str;

pub fn create_topscores_dialog(scores: &Scores, messages: &Messages, highlight: Option<usize>) -> Result<Container<()>> {
    let rect = Rect::new(240, 125, 320, 350);
    let bg = BackgroundPattern::Blue;

    let mut container = Container::<()>::modal(rect, bg);

    container.push(WidgetMapAction::no_action(
        Label::title(Rect::new(10, 10, 300, 40), messages.top_scores)
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
        DialogButton::new(Rect::new(115, 310, 90, 25), bg, messages.ok, Some(Key::Escape), ())
    );

    Ok(container)
}
