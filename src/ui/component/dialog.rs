use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::container::Container;

#[derive(Clone)]
pub enum DialogResult<T> {
    Ok(T),
    Cancel,
}

pub fn dialod_widget<B: Into<Option<Background>>, A, W: Widget<A> + 'static>(background: B, widget: W) -> Container<A> {
    let screen = Size::new(800, 600);
    let widget_size = widget.get_size();
    let x = screen.width.wrapping_sub(widget_size.width) / 2;
    let y = screen.height.wrapping_sub(widget_size.height) / 2;
    Container::<A>::modal(screen, background)
        .add(x, y, widget)
}
