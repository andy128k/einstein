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
    Image(&'static Resource, Option<Rect>),
}

#[derive(Clone, Copy)]
pub struct FontSize(pub u16);

impl FontSize {
    pub const TEXT: FontSize = FontSize(16);
    pub const BUTTON: FontSize = FontSize(14);
    pub const TITLE: FontSize = FontSize(26);
}
