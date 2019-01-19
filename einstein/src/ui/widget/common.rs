use sdl2::pixels::Color;
use crate::ui::common::Rect;
use crate::resources::manager::Resource;

#[derive(Clone, Copy)]
pub enum Border {
    Beveled(Color, Color),
    Etched(Color, Color),
}

#[derive(Clone, Copy)]
pub enum Background {
    Color(Color),
    Image(&'static Resource, bool, Option<Rect>),
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
