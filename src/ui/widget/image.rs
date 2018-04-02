use sdl::video::Surface;
use error::*;
use ui::context::{Context, Rect};
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
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context) -> Result<()> {
        context.tiles(&self.image);
        Ok(())
    }
}
