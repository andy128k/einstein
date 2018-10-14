use error::*;
use ui::context::{Context, Rect};
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use resources::manager::ResourceManager;

pub struct Window {
    rect: Rect,
    background: BackgroundPattern,
}

impl Window {
    pub fn new(rect: Rect, background: BackgroundPattern) -> Self {
        Self { rect, background }
    }
}

impl Widget<Nothing> for Window {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager) -> Result<()> {
        let bg = self.background.load(resource_manager);
        context.tiles(bg);
        context.bevel(true, 1);
        Ok(())
    }
}
