use sdl2::pixels::Color;
use ui::context::Rect;
use resources::manager::Resource;

#[derive(Clone, Copy)]
pub enum Border {
    Raised,
    Sunken,
    Etched,
}

#[derive(Clone, Copy)]
pub enum BackgroundPattern {
    Color(Color),
    Blue,
    BlueHighlighted,
    Green,
    GreenHighlighted,
    White,
    WhiteHighlighted,
    Red,
    RedHighlighted,
    Custom(&'static Resource, bool),
    Sprite(&'static Resource, bool, Rect),
}

fn gamma(v: u8, k: f64) -> u8 {
    (255.0 * (v as f64 / 255.0).powf(1.0 / k)).round().min(255.0) as u8
}

const GAMMA_K: f64 = 1.5;

impl BackgroundPattern {
    pub fn highlighted(self) -> Self {
        match self {
            BackgroundPattern::Color(Color { r, g, b, a }) => BackgroundPattern::Color(Color {
                r: gamma(r, GAMMA_K),
                g: gamma(g, GAMMA_K),
                b: gamma(b, GAMMA_K),
                a: a,
            }),
            BackgroundPattern::Blue => BackgroundPattern::BlueHighlighted,
            BackgroundPattern::Green => BackgroundPattern::GreenHighlighted,
            BackgroundPattern::White => BackgroundPattern::WhiteHighlighted,
            BackgroundPattern::Red => BackgroundPattern::RedHighlighted,
            BackgroundPattern::Custom(resource, _) => BackgroundPattern::Custom(resource, true),
            BackgroundPattern::Sprite(resource, _, rect) => BackgroundPattern::Sprite(resource, true, rect),
            other => other,
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
