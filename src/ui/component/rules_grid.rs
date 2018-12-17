use std::rc::Rc;
use crate::cell::RefCell;
use crate::ui::context::Size;
use crate::ui::widget::container::Container;
use crate::ui::layout::grid::GridBuilder;
use crate::ui::component::game::{GamePrivate};
use crate::ui::component::rule::RuleWidget;
use crate::ui::rule::{TILE_WIDTH, TILE_HEIGHT};

pub fn create_horizontal_rules(size: Size, state: Rc<RefCell<GamePrivate>>) -> Container<usize> {
    let container = Container::container(size, None, None);
    let mut grid = GridBuilder::new(container, 3, 8);
    let rule_size = Size::new(TILE_WIDTH * 3, TILE_HEIGHT);
    for i in 0..24 {
        let rule_index = state.borrow().horizontal_rules.get(i).map(|index| *index);
        grid = grid.add(
            i % 3, i / 3,
            RuleWidget::new(rule_size, state.clone(), rule_index)
        );
    }
    grid.build()
}

pub fn create_vertical_rules(size: Size, state: Rc<RefCell<GamePrivate>>) -> Container<usize> {
    let container = Container::container(size, None, None);
    let mut grid = GridBuilder::new(container, 15, 1);
    let rule_size = Size::new(TILE_WIDTH, TILE_HEIGHT * 2);
    for i in 0..15 {
        let rule_index = state.borrow().vertical_rules.get(i).map(|index| *index);
        grid = grid.add(
            i, 0,
            RuleWidget::new(rule_size, state.clone(), rule_index)
        );
    }
    grid.build()
}
