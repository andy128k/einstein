use sdl2::pixels::Color;
use ui::context::Rect;
use resources::manager::Resource;
use resources::background::{BLUE_PATTERN, GREEN_PATTERN, MARBLE_PATTERN, RED_PATTERN};

#[derive(Clone, Copy)]
pub enum Border {
    Raised,
    Sunken,
    Etched,
}

#[derive(Clone, Copy)]
pub enum Background {
    Color(Color),
    Pattern(&'static Resource, bool),
    Sprite(&'static Resource, bool, Rect),
}

impl Background {
    pub const BLUE_PATTERN: Self = Background::Pattern(&BLUE_PATTERN, false);
    pub const GREEN_PATTERN: Self = Background::Pattern(&GREEN_PATTERN, false);
    pub const WHITE_PATTERN: Self = Background::Pattern(&MARBLE_PATTERN, false);
    pub const RED_PATTERN: Self = Background::Pattern(&RED_PATTERN, false);
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
            Background::Pattern(resource, _) => Background::Pattern(resource, true),
            Background::Sprite(resource, _, rect) => Background::Sprite(resource, true, rect),
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
