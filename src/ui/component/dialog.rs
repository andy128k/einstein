use std::rc::Rc;
use crate::cell::RefCell;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::Background;
use crate::ui::widget::container::Container;
use crate::ui::widget::conditional::ConditionalWidget;

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

pub fn dialod_widget<B: Into<Option<Background>>, A, W: Widget<A> + 'static>(background: B, widget: W) -> Container<A> {
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
    WidgetMapAction::new(
        ConditionalWidget::new(
            condition.clone(),
            factory
        ),
        move |action, _| {
            *condition2.borrow_mut() = None;
            Ok(EventReaction::action(action.clone()))
        }
    )
}
