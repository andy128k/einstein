use sdl2::pixels::Color;
use ui::widget::common::*;
use ui::context::{Context, Rect, HorizontalAlign, VerticalAlign};
use resources::manager::ResourceManager;
use resources::background::{BLUE_PATTERN, GREEN_PATTERN, MARBLE_PATTERN, RED_PATTERN};
use resources::fonts::*;

pub struct Text {
    text: String,
    font_size: FontSize,
    color: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    shadow: bool,
}

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            font_size: FontSize::Text,
            color: Color::RGB(0, 0, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
            shadow: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn font_size(mut self, font_size: FontSize) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn shadow(mut self) -> Self {
        self.shadow = true;
        self
    }

    pub fn halign(mut self, horizontal_align: HorizontalAlign) -> Self {
        self.horizontal_align = horizontal_align;
        self
    }

    pub fn valign(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = vertical_align;
        self
    }
}

pub struct Brick {
    rect: Rect,
    background: Option<BackgroundPattern>,
    text: Option<Text>,
    border: Option<Border>,
    children: Vec<Brick>,
}

impl Brick {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            text: None,
            background: None,
            border: None,
            children: vec![],
        }
    }

    pub fn text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    pub fn background(mut self, background: BackgroundPattern) -> Self {
        self.background = Some(background);
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub fn add(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn draw(&self, context: &Context, resource_manager: &mut ResourceManager) -> Result<(), ::failure::Error> {
        match self.background {
            Some(BackgroundPattern::Color(color)) => {
                context.fill(color)
            },
            Some(BackgroundPattern::Blue) => {
                let image = resource_manager.image("BLUE_PATTERN", BLUE_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::BlueHighlighted) => {
                let image = resource_manager.image_highlighted("BLUE_PATTERN", BLUE_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::Green) => {
                let image = resource_manager.image("GREEN_PATTERN", GREEN_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::GreenHighlighted) => {
                let image = resource_manager.image_highlighted("GREEN_PATTERN", GREEN_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::White) => {
                let image = resource_manager.image("MARBLE_PATTERN", MARBLE_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::WhiteHighlighted) => {
                let image = resource_manager.image_highlighted("MARBLE_PATTERN", MARBLE_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::Red) => {
                let image = resource_manager.image("RED_PATTERN", RED_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::RedHighlighted) => {
                let image = resource_manager.image_highlighted("RED_PATTERN", RED_PATTERN);
                context.tiles(image);
            },
            Some(BackgroundPattern::Custom(name, bytes)) => {
                let image = resource_manager.image(name, bytes);
                context.tiles(image);
            },
            None => {},
        }
        if let Some(ref text) = self.text {
            let font = match text.font_size {
                FontSize::Text => text_font()?,
                FontSize::Button => button_font()?,
                FontSize::Menu => menu_font()?,
                FontSize::Title => title_font()?,
            };
            context.text(&text.text, font, text.color, text.shadow, text.horizontal_align, text.vertical_align)?;
        }
        match self.border {
            Some(Border::Raised) => context.bevel(true, 1),
            Some(Border::Sunken) => context.bevel(false, 1),
            Some(Border::Etched) => context.etched(),
            None => {},
        }
        for child in &self.children {
            let child_context = context.relative(child.rect);
            child.draw(&child_context, resource_manager)?;
        }
        Ok(())
    }
}
