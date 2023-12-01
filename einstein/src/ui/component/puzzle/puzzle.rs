use crate::cell::RefCell;
use crate::ui::common::Size;
use crate::ui::component::game::GamePrivate;
use crate::ui::component::puzzle::puzzle_cell::{PuzzleAction, PuzzleCell};
use crate::ui::layout::grid::GridBuilder;
use crate::ui::widget::container::Container;
use einstein_puzzle::rules::{Kind, PuzzleSize};
use std::rc::Rc;

const WIDTH: u32 = 308;
const HEIGHT: u32 = 308;

pub fn new_puzzle_widget(state: &Rc<RefCell<GamePrivate>>) -> Container<PuzzleAction> {
    let container = Container::container(Size::new(WIDTH, HEIGHT), None, None);
    let PuzzleSize { kinds, values } = state.borrow().solved_puzzle.size();
    let mut grid = GridBuilder::new(container, usize::from(values), usize::from(kinds));
    for row in 0..kinds {
        let kind = Kind(row);
        for col in 0..values {
            let cell = PuzzleCell::new(state, kind, col);
            grid = grid.add(usize::from(col), usize::from(row), cell);
        }
    }

    grid.build()
}
