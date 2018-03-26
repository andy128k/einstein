use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::utils::{load_image, draw_tiles, draw_bevel};

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

impl Widget for Window {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_tiles(&surface, self.rect, &self.background);
        draw_bevel(&surface, self.rect, true, 1);
        Ok(())
    }
}
