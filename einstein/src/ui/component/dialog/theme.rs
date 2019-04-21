use sdl2::pixels::Color;
use crate::ui::widget::common::Background;
use crate::resources::background::{
    BLUE_PATTERN,
    BLUE_PATTERN_HIGHLIGHTED,
    GREEN_PATTERN,
    GREEN_PATTERN_HIGHLIGHTED,
    MARBLE_PATTERN,
    MARBLE_PATTERN_HIGHLIGHTED,
    RED_PATTERN,
    RED_PATTERN_HIGHLIGHTED,
};

#[derive(Clone, Copy)]
pub enum DialogTheme {
    Blue,
    Green,
    White,
    Red,
}

impl DialogTheme {
    pub fn background(&self, highlighted: bool) -> Background {
        match (self, highlighted) {
            (DialogTheme::Blue, false) => Background::Image(&BLUE_PATTERN, None),
            (DialogTheme::Blue, true) => Background::Image(&BLUE_PATTERN_HIGHLIGHTED, None),
            (DialogTheme::Green, false) => Background::Image(&GREEN_PATTERN, None),
            (DialogTheme::Green, true) => Background::Image(&GREEN_PATTERN_HIGHLIGHTED, None),
            (DialogTheme::White, false) => Background::Image(&MARBLE_PATTERN, None),
            (DialogTheme::White, true) => Background::Image(&MARBLE_PATTERN_HIGHLIGHTED, None),
            (DialogTheme::Red, false) => Background::Image(&RED_PATTERN, None),
            (DialogTheme::Red, true) => Background::Image(&RED_PATTERN_HIGHLIGHTED, None),
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            DialogTheme::Blue => Color::RGB(255, 255, 0),
            DialogTheme::Green => Color::RGB(255, 255, 0),
            DialogTheme::White => Color::RGB(255, 0, 0),
            DialogTheme::Red => Color::RGB(255, 255, 255),
        }
    }

    pub fn colors3d(&self) -> (Color, Color) {
        match self {
            DialogTheme::Blue => (Color::RGB(200, 235, 255), Color::RGB(50, 50, 70)),
            DialogTheme::Green => (Color::RGB(136, 160, 140), Color::RGB(50, 70, 50)),
            DialogTheme::White => (Color::RGB(255, 255, 255), Color::RGB(70, 70, 70)),
            DialogTheme::Red => (Color::RGB(200, 160, 160), Color::RGB(70, 50, 50)),
        }
    }
}
