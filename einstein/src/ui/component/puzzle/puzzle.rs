use crate::cell::RefCell;
use crate::ui::common::Size;
use crate::ui::component::game::GamePrivate;
use crate::ui::component::puzzle::puzzle_cell::{PuzzleAction, PuzzleCell};
use crate::ui::layout::grid::GridBuilder;
use crate::ui::widget::container::Container;
use einstein_puzzle::rules::{Kind, Value, PUZZLE_SIZE};
use std::rc::Rc;

const WIDTH: u32 = 308;
const HEIGHT: u32 = 308;

pub fn new_puzzle_widget(state: &Rc<RefCell<GamePrivate>>) -> Container<PuzzleAction> {
    let container = Container::container(Size::new(WIDTH, HEIGHT), None, None);
    let mut grid = GridBuilder::new(container, PUZZLE_SIZE, PUZZLE_SIZE);

    for (row_index, row) in Kind::values().iter().enumerate() {
        for col in 0..Value::COUNT {
            let cell = PuzzleCell::new(state, *row, col as u8);
            grid = grid.add(col as usize, row_index, cell);
        }
    }

    grid.build()
}
