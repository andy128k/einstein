use sdl2::pixels::Color;
use crate::ui::widget::common::Background;
use crate::resources::background::{BLUE_PATTERN, GREEN_PATTERN, MARBLE_PATTERN, RED_PATTERN};

#[derive(Clone, Copy)]
pub enum DialogTheme {
    Blue,
    Green,
    White,
    Red,
}

impl DialogTheme {
    pub fn background(&self, highlighted: bool) -> Background {
        match self {
            DialogTheme::Blue => Background::Image(&BLUE_PATTERN, highlighted, None),
            DialogTheme::Green => Background::Image(&GREEN_PATTERN, highlighted, None),
            DialogTheme::White => Background::Image(&MARBLE_PATTERN, highlighted, None),
            DialogTheme::Red => Background::Image(&RED_PATTERN, highlighted, None),
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
            DialogTheme::Blue => (Color::RGB(185, 220, 255), Color::RGB(90, 125, 165)),
            DialogTheme::Green => (Color::RGB(136, 160, 140), Color::RGB(30, 55, 35)),
            DialogTheme::White => (Color::RGB(255, 255, 255), Color::RGB(128, 128, 128)),
            DialogTheme::Red => (Color::RGB(255, 255, 255), Color::RGB(128, 128, 128)),
        }
    }
}
