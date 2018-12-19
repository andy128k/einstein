use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign, VerticalAlign};
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::widget::widget::*;
use crate::resources::manager::ResourceManager;

pub struct Label {
    size: Size,
    text: String,
    font_size: FontSize,
    color: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
}

impl Label {
    pub fn new(size: Size, text: &str, color: Color, horizontal_align: HorizontalAlign) -> Self {
        Self {
            size,
            text: text.to_string(),
            font_size: FontSize::TEXT,
            color,
            horizontal_align,
            vertical_align: VerticalAlign::Middle,
        }
    }

    pub fn title(size: Size, text: &str) -> Self {
        Self {
            size,
            text: text.to_string(),
            font_size: FontSize::TITLE,
            color: Color::RGB(255, 255, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
        }
    }
}

impl Widget<Nothing> for Label {
    fn get_size(&self) -> Size { self.size }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        Brick::new(self.get_size().width, self.get_size().height)
            .text(Text::new(&self.text)
                .font_size(self.font_size)
                .color(self.color)
                .shadow()
                .halign(self.horizontal_align)
                .valign(self.vertical_align))
    }
}
