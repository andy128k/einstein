use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::utils::{load_image, draw_tiles};

pub struct Image {
    rect: Rect,
    image: Surface,
}

impl Image {
    pub fn new(rect: Rect, bg: &[u8]) -> Result<Self> {
        let image = load_image(bg)?;
        Ok(Self { rect, image })
    }
}

impl Widget for Image {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_tiles(surface, self.rect, &self.image);
        Ok(())
    }
}
