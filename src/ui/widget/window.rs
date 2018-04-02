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
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context) -> Result<()> {
        let c = context.relative(self.rect);
        c.tiles(&self.background);
        c.bevel(true, 1);
        Ok(())
    }
}
