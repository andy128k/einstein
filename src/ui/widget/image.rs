use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::context::Context;
use ui::widget::widget::*;
use ui::utils::load_image;

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

impl Widget<Nothing> for Image {
    fn draw(&self, context: &Context) -> Result<()> {
        let c = context.relative(self.rect);
        c.tiles(&self.image);
        Ok(())
    }
}
