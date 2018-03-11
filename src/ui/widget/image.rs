use failure::err_msg;
use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::utils::{tiled_image, draw_bevel};

pub struct Image {
    rect: Rect,
    image: Surface,
}

impl Image {
    pub fn new(rect: Rect, bg: &[u8]) -> Result<Self> {
        let win = tiled_image(bg, rect.width() as u16, rect.height() as u16)?;

        let image = win.display_format().map_err(err_msg)?;
        Ok(Self { rect, image })
    }
}

impl Widget for Image {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        surface.blit_at(&self.image, self.rect.left() as i16, self.rect.top() as i16);
        Ok(())
    }
}
