use sdl::video::Surface;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use error::*;
use ui::widget::widget::*;
use ui::utils::{load_image, draw_tiles, draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::title_font;

pub struct Background {
    rect: Rect,
    background: Surface,
    title_background: Surface,
}

const RAIN_TILE: &[u8] = include_bytes!("./rain.bmp");
const TITLE_BG: &[u8] = include_bytes!("./title.bmp");

impl Background {
    pub fn new() -> Result<Self> {
        let rect = Rect::new(0, 0, 800, 600);
        let background = load_image(RAIN_TILE)?;
        let title_background = load_image(TITLE_BG)?;
        Ok(Self { rect, background, title_background })
    }
}

impl Widget for Background {
    fn get_rect(&self) -> Rect { self.rect }

    fn draw(&self, surface: &Surface) -> Result<()> {
        draw_tiles(surface, self.get_rect(), &self.background);
        surface.blit_at(&self.title_background, 8, 10);
        let text = "Einstein Puzzle"; // i18n msg(L"einsteinPuzzle")
        draw_text(surface, text, title_font()?, Color::RGB(255, 255, 0), true, Rect::new(20, 10, 500, 47), HorizontalAlign::Left, VerticalAlign::Middle)?;
        Ok(())
    }
}
