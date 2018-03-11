use std::rc::Rc;
use debug_cell::RefCell;
use sdl2::rect::{Rect};
use rules::{PUZZLE_SIZE};
use ui::widget::container::Container;
use ui::thing::ThingImages;
use ui::component::game::GamePrivate;
use ui::component::puzzle::puzzle_cell::PuzzleCell;
use error::*;

const FIELD_OFFSET_X: u16 =    12;
const FIELD_OFFSET_Y: u16 =    68;
const FIELD_GAP_X: u16 =        4;
const FIELD_GAP_Y: u16 =        4;
const FIELD_TILE_WIDTH: u16 =  48;
const FIELD_TILE_HEIGHT: u16 = 48;

pub fn new_puzzle_widget(state: Rc<RefCell<GamePrivate>>) -> Result<Container<()>> {
    let rect = Rect::new(
        FIELD_OFFSET_X as i32,
        FIELD_OFFSET_Y as i32,
        (6 * FIELD_TILE_WIDTH + 5 * FIELD_GAP_X) as u32,
        (6 * FIELD_TILE_HEIGHT + 5 * FIELD_GAP_Y) as u32
    );

    let thing_images = ThingImages::new()?;

    let mut container = Container::new(rect, ());

    for row in 0..PUZZLE_SIZE {
        for col in 0..PUZZLE_SIZE {
            let cell = PuzzleCell::new(state.clone(), row as u8, col as u8, thing_images.clone())?;
            container.add(Box::new(cell));
        }
    }

    Ok(container)
}
