use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::label::*;
use crate::ui::widget::dialog_button::*;
use crate::ui::widget::container::Container;
use crate::ui::layout::grid::GridBuilder;
use crate::ui::component::dialog::dialod_widget;
use crate::resources::messages::Messages;
use crate::storage::{Scores};
use crate::util::time::sec_to_str;

pub fn create_topscores_dialog(scores: &Scores, messages: &Messages, highlight: Option<usize>) -> Container<()> {
    let bg = Background::BLUE_PATTERN;

    let container = Container::<()>::container(Size::new(320, 350), bg, Border::Raised)
        .add(10, 10,
            Label::title(Size::new(300, 40), messages.top_scores).no_action()
        )
        .add(10, 50, {
            let mut grid = GridBuilder::new(Container::container(Size::new(300, 250), None, None), 3, 1);
            for (i, score) in scores.0.iter().enumerate() {
                let color = if highlight == Some(i) {
                    Color::RGB(255, 255, 0)
                } else {
                    Color::RGB(255, 255, 255)
                };

                grid = grid
                    .add(0, i,
                        Label::new(Size::new(30, 25), &format!("{}.", i + 1), color, HorizontalAlign::Right).no_action()
                    )
                    .add(1, i,
                        Label::new(Size::new(160, 25), &score.name, color, HorizontalAlign::Left).no_action()
                    )
                    .add(2, i,
                        Label::new(Size::new(80, 25), &sec_to_str(score.score), color, HorizontalAlign::Right).no_action()
                    );
            }
            grid.build()
        })
        .add(115, 310,
            DialogButton::new(Size::new(90, 25), bg, messages.ok, Some(Keycode::Escape), ())
        );

    dialod_widget(None, container)
}
