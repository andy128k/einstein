use std::rc::Rc;
use crate::cell::RefCell;
use crate::ui::context::Size;
use crate::ui::widget::container::Container;
use crate::ui::widget::grid::new_grid;
use crate::ui::component::game::{GamePrivate};
use crate::ui::component::rule::RuleWidget;
use crate::ui::rule::{TILE_WIDTH, TILE_HEIGHT};

pub fn create_horizontal_rules(size: Size, state: Rc<RefCell<GamePrivate>>) -> Container<usize> {
    let rule_size = Size::new(TILE_WIDTH * 3, TILE_HEIGHT);

    let mut children = Vec::new();
    for i in 0..24 {
        let rule_index = state.borrow().horizontal_rules.get(i).map(|index| *index);
        children.push(
            RuleWidget::new(rule_size, state.clone(), rule_index)
        );
    }
    new_grid(size, rule_size, 3, 8, children)
}

pub fn create_vertical_rules(size: Size, state: Rc<RefCell<GamePrivate>>) -> Container<usize> {
    let rule_size = Size::new(TILE_WIDTH, TILE_HEIGHT * 2);

    let mut children = Vec::new();
    for i in 0..15 {
        let rule_index = state.borrow().vertical_rules.get(i).map(|index| *index);
        children.push(
            RuleWidget::new(rule_size, state.clone(), rule_index)
        );
    }
    new_grid(size, rule_size, 15, 1, children)
}
