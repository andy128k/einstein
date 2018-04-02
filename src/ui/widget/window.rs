use sdl::video::Surface;
use error::*;
use ui::context::{Context, Rect};
use ui::widget::widget::*;
use ui::utils::load_image;

pub struct Window {
    rect: Rect,
    background: Surface,
}

impl Window {
    pub fn new(rect: Rect, bg: &[u8]) -> Result<Self> {
        let background = load_image(bg)?;
        Ok(Self { rect, background })
    }
}

impl Widget<Nothing> for Window {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context) -> Result<()> {
        context.tiles(&self.background);
        context.bevel(true, 1);
        Ok(())
    }
}
