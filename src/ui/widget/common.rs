use sdl::video::Surface;
use resources::manager::ResourceManager;
use resources::background::{BLUE_PATTERN, GREEN_PATTERN, MARBLE_PATTERN, RED_PATTERN};

#[derive(Clone, Copy)]
pub enum BackgroundPattern {
    Blue,
    BlueHighlighted,
    Green,
    GreenHighlighted,
    White,
    WhiteHighlighted,
    Red,
    RedHighlighted,
    Custom(&'static str, &'static [u8]),
}

impl BackgroundPattern {
    pub fn highlighted(&self) -> Self {
        match self {
            BackgroundPattern::Blue => BackgroundPattern::BlueHighlighted,
            BackgroundPattern::Green => BackgroundPattern::GreenHighlighted,
            BackgroundPattern::White => BackgroundPattern::WhiteHighlighted,
            BackgroundPattern::Red => BackgroundPattern::RedHighlighted,
            other => *other,
        }
    }

    pub fn load<'r>(&self, resource_manager: &'r mut ResourceManager) -> &'r Surface {
        match self {
            BackgroundPattern::Blue => resource_manager.image("BLUE_PATTERN", BLUE_PATTERN),
            BackgroundPattern::BlueHighlighted => resource_manager.image_highlighted("BLUE_PATTERN", BLUE_PATTERN),
            BackgroundPattern::Green => resource_manager.image("GREEN_PATTERN", GREEN_PATTERN),
            BackgroundPattern::GreenHighlighted => resource_manager.image_highlighted("GREEN_PATTERN", GREEN_PATTERN),
            BackgroundPattern::White => resource_manager.image("MARBLE_PATTERN", MARBLE_PATTERN),
            BackgroundPattern::WhiteHighlighted => resource_manager.image_highlighted("MARBLE_PATTERN", MARBLE_PATTERN),
            BackgroundPattern::Red => resource_manager.image("RED_PATTERN", RED_PATTERN),
            BackgroundPattern::RedHighlighted => resource_manager.image_highlighted("RED_PATTERN", RED_PATTERN),
            BackgroundPattern::Custom(name, bytes) => resource_manager.image(name, bytes),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FontSize {
    Text,
    Button,
    Menu,
    Title,
}
