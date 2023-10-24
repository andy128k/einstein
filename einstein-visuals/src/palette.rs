use einstein_puzzle::rules::Kind;
pub use palette::LinSrgb as Color;
use palette::{Darken, Lighten};
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub enum PaletteColor {
    Kind0 = 0,
    Kind1,
    Kind2,
    Kind3,
    Kind4,
    Kind5,
    Kind6,
    Kind7,
    Kind8,
    KindDefault,
    Background,
    Stroke,
    Near,
    Direction,
    Under,
    Between,
    Border,
    Selection,
}

impl PaletteColor {
    pub fn for_kind(kind: Kind) -> Self {
        match kind.0 {
            0 => Self::Kind0,
            1 => Self::Kind1,
            2 => Self::Kind2,
            3 => Self::Kind3,
            4 => Self::Kind4,
            5 => Self::Kind5,
            6 => Self::Kind6,
            7 => Self::Kind7,
            8 => Self::Kind8,
            _ => Self::KindDefault,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ColorModifcation {
    Normal = 0,
    Faded,
}

pub struct Palette([[Color; 2]; 18]);

impl Palette {
    pub fn get(&self, color: PaletteColor, modification: ColorModifcation) -> &Color {
        &self.0[color as usize][modification as usize]
    }
}

pub fn default_palette() -> &'static Palette {
    static PALETTE: OnceLock<Palette> = OnceLock::new();
    PALETTE.get_or_init(|| {
        let color_kind0 = Color::new(1.0, 0.68, 0.68);
        let color_kind1 = Color::new(1.0, 0.84, 0.65);
        let color_kind2 = Color::new(0.99, 1.0, 0.71);
        let color_kind3 = Color::new(0.79, 1.0, 0.75);
        let color_kind4 = Color::new(0.61, 0.96, 1.0);
        let color_kind5 = Color::new(0.63, 0.77, 1.0);
        let color_kind6 = Color::new(0.74, 0.7, 1.0);
        let color_kind7 = Color::new(1.0, 0.78, 1.0);
        let color_kind8 = Color::new(1.0, 1.0, 0.99);
        let color_kind_default = Color::new(1.0, 1.0, 1.0);
        let color_background = Color::new(0.95, 0.95, 0.95);
        let color_stroke = Color::new(0.25, 0.25, 0.25);
        let color_near = Color::new(1.0, 0.0, 0.0);
        let color_direction = Color::new(1.0, 0.0, 0.0);
        let color_under = Color::new(1.0, 0.0, 1.0);
        let color_between = Color::new(0.0, 0.0, 1.0);
        let color_border = Color::new(0.5, 0.5, 0.5);
        let color_selection = Color::new(0.21, 0.52, 0.89);

        Palette([
            [color_kind0, color_kind0.lighten(0.5)],
            [color_kind1, color_kind1.lighten(0.5)],
            [color_kind2, color_kind2.lighten(0.5)],
            [color_kind3, color_kind3.lighten(0.5)],
            [color_kind4, color_kind4.lighten(0.5)],
            [color_kind5, color_kind5.lighten(0.5)],
            [color_kind6, color_kind6.lighten(0.5)],
            [color_kind7, color_kind7.lighten(0.5)],
            [color_kind8, color_kind8.lighten(0.5)],
            [color_kind_default, color_kind_default.darken(0.9)],
            [color_background, color_background.darken(0.5)],
            [color_stroke, color_stroke.lighten(0.5)],
            [color_near, color_near.lighten(0.5)],
            [color_direction, color_direction.lighten(0.5)],
            [color_under, color_under.lighten(0.5)],
            [color_between, color_between.lighten(0.5)],
            [color_border, color_border.lighten(0.5)],
            [color_selection, color_selection],
        ])
    })
}
