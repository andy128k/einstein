use error::*;
use ui::context::{Context, Rect};
use ui::widget::widget::*;
use ui::widget::common::{BackgroundPattern, Border};
use ui::widget::brick::Brick;
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
        let brick = Brick::new(self.get_rect())
            .background(self.background)
            .border(Border::Raised);
        brick.draw(context, resource_manager)?;
        Ok(())
    }
}
