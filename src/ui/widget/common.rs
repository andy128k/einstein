use sdl2::pixels::Color;
use crate::ui::common::Rect;
use crate::resources::manager::Resource;
use crate::resources::background::{BLUE_PATTERN, GREEN_PATTERN, MARBLE_PATTERN, RED_PATTERN};

#[derive(Clone, Copy)]
pub enum Border {
    Raised,
    Sunken,
    Etched,
}

#[derive(Clone, Copy)]
pub enum Background {
    Color(Color),
    Image(&'static Resource, bool, Option<Rect>),
}

impl Background {
    pub const BLUE_PATTERN: Self = Background::Image(&BLUE_PATTERN, false, None);
    pub const GREEN_PATTERN: Self = Background::Image(&GREEN_PATTERN, false, None);
    pub const WHITE_PATTERN: Self = Background::Image(&MARBLE_PATTERN, false, None);
    pub const RED_PATTERN: Self = Background::Image(&RED_PATTERN, false, None);
}

fn gamma(v: u8, k: f64) -> u8 {
    (255.0 * (v as f64 / 255.0).powf(1.0 / k)).round().min(255.0) as u8
}

const GAMMA_K: f64 = 1.5;

impl Background {
    pub fn highlighted(self) -> Self {
        match self {
            Background::Color(Color { r, g, b, a }) => Background::Color(Color {
                r: gamma(r, GAMMA_K),
                g: gamma(g, GAMMA_K),
                b: gamma(b, GAMMA_K),
                a: a,
            }),
            Background::Image(resource, _, rect) => Background::Image(resource, true, rect),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FontSize(pub u16);

impl FontSize {
    pub const TEXT: FontSize = FontSize(16);
    pub const BUTTON: FontSize = FontSize(14);
    pub const TITLE: FontSize = FontSize(26);
}
