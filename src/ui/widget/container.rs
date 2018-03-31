use sdl::video::Surface;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;

pub struct Container<T> {
    rect: Rect,
    widgets: Vec<Box<Widget>>,
    pub private: T
}

impl<T> Container<T> {
    pub fn new(rect: Rect, private: T) -> Self {
        Self {
            rect,
            widgets: Vec::new(),
            private,
        }
    }

    pub fn add(&mut self, widget: Box<Widget>) {
        self.widgets.push(widget);
    }
}

impl<T> Widget for Container<T> {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&self, event: &Event) -> Option<Effect> {
        for widget in &self.widgets {
            let effect = widget.on_event(event);
            if effect.is_some() {
                return effect;
            }
        }
        None
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        for widget in &self.widgets {
            widget.draw(surface)?;
        }
        Ok(())
    }

    fn draw_in_rects(&self, surface: &Surface, rects: &[Rect]) -> Result<()> {
        for widget in &self.widgets {
            widget.draw_in_rects(surface, rects)?;
        }
        Ok(())
    }
}
