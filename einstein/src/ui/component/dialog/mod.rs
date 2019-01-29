pub mod theme;
pub mod button;
pub mod checkbox;
pub mod slider;

use std::rc::Rc;
use crate::cell::RefCell;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::{Background, Border};
use crate::ui::widget::container::Container;
use crate::ui::widget::conditional::ConditionalWidget;
pub use self::theme::*;
pub use self::button::*;
pub use self::checkbox::*;
pub use self::slider::*;

#[derive(Clone)]
pub enum DialogResult<T> {
    Ok(T),
    Cancel,
}

impl<T> DialogResult<T> {
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> DialogResult<U> {
        match self {
            DialogResult::Ok(v) => DialogResult::Ok(f(v)),
            DialogResult::Cancel => DialogResult::Cancel,
        }
    }
}

pub fn dialog_container<T>(size: Size, theme: DialogTheme) -> Container<T> {
    let (color1, color2) = theme.colors3d();
    let border = Border::Beveled(color1, color2);
    Container::container(size, theme.background(false), border)
}

pub fn dialog_widget<B: Into<Option<Background>>, A, W: Widget<A> + 'static>(background: B, widget: W) -> Container<A> {
    let screen = Size::new(800, 600);
    let widget_size = widget.get_size();
    let x = screen.width.wrapping_sub(widget_size.width) / 2;
    let y = screen.height.wrapping_sub(widget_size.height) / 2;
    Container::<A>::modal(screen, background)
        .add(x, y, widget)
}

pub fn cond_dialog<I, F, W, A>(condition: &Rc<RefCell<Option<I>>>, factory: F) -> impl Widget<A>
    where
        I: 'static,
        F: Fn(&I) -> W + 'static,
        W: Widget<A> + 'static,
        A: Clone,
{
    let condition2 = condition.clone();
    ConditionalWidget::new(
        condition.clone(),
        factory
    ).flat_map_action(move |action, _| {
        *condition2.borrow_mut() = None;
        Ok(EventReaction::action(action.clone()))
    })
}
