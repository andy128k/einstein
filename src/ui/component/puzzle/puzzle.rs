use std::rc::Rc;
use crate::cell::RefCell;
use crate::rules::{PUZZLE_SIZE};
use crate::ui::context::Size;
use crate::ui::widget::container::Container;
use crate::ui::component::game::GamePrivate;
use crate::ui::component::puzzle::puzzle_cell::{PuzzleCell, PuzzleAction};

const FIELD_GAP_X: u16 =        4;
const FIELD_GAP_Y: u16 =        4;
const FIELD_TILE_WIDTH: u16 =  48;
const FIELD_TILE_HEIGHT: u16 = 48;

const WIDTH: u32 = (6 * FIELD_TILE_WIDTH + 5 * FIELD_GAP_X) as u32;
const HEIGHT: u32 = (6 * FIELD_TILE_HEIGHT + 5 * FIELD_GAP_Y) as u32;

pub fn new_puzzle_widget(state: &Rc<RefCell<GamePrivate>>) -> Container<PuzzleAction> {
    let mut container = Container::container(Size::new(WIDTH, HEIGHT), None);

    for row in 0..PUZZLE_SIZE {
        for col in 0..PUZZLE_SIZE {
            let cell_x = (col as i32) * ((FIELD_TILE_WIDTH + FIELD_GAP_X) as i32);
            let cell_y = (row as i32) * ((FIELD_TILE_HEIGHT + FIELD_GAP_Y) as i32);

            let cell = PuzzleCell::new(state, row as u8, col as u8);
            container.push(cell_x as u32, cell_y as u32, cell);
        }
    }

    container
}
