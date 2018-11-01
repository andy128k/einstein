use sdl2::pixels::Color;
use ui::context::{Rect, HorizontalAlign, VerticalAlign};
use ui::widget::common::*;
use ui::widget::brick::*;
use ui::widget::widget::*;
use resources::manager::ResourceManager;

pub struct Label {
    text: String,
    rect: Rect,
    font_size: FontSize,
    color: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
}

impl Label {
    pub fn new(rect: Rect, text: &str, color: Color, horizontal_align: HorizontalAlign) -> Self {
        Self {
            text: text.to_string(),
            rect,
            font_size: FontSize::TEXT,
            color,
            horizontal_align,
            vertical_align: VerticalAlign::Middle,
        }
    }

    pub fn title(rect: Rect, text: &str) -> Self {
        Self {
            text: text.to_string(),
            rect,
            font_size: FontSize::TITLE,
            color: Color::RGB(255, 255, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        }
    }
}

impl Widget<Nothing> for Label {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        Brick::new(self.get_rect())
            .text(Text::new(&self.text)
                .font_size(self.font_size)
                .color(self.color)
                .shadow()
                .halign(self.horizontal_align)
                .valign(self.vertical_align))
    }
}
