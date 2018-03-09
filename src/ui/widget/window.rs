use failure::err_msg;
use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::utils::{tiled_image, draw_bevel};

pub struct Window {
    rect: Rect,
    background: Surface,
}

impl Window {
    pub fn new(rect: Rect, bg: &[u8]) -> Result<Self> {
        let mut win = tiled_image(bg, rect.width() as u16, rect.height() as u16)?;

        win.lock();
        let mut bounding_rect = rect;
        bounding_rect.reposition((0, 0));
        draw_bevel(&mut win, bounding_rect, true, 1);
        win.unlock();

        let background = win.display_format().map_err(err_msg)?;
        Ok(Self { rect, background })
    }
}

impl Widget for Window {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        surface.blit_at(&self.background, self.rect.left() as i16, self.rect.top() as i16);
        Ok(())
    }
}
