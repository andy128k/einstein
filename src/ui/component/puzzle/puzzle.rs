use std::rc::Rc;
use debug_cell::RefCell;
use rules::{PUZZLE_SIZE};
use ui::context::{Context, Rect};
use ui::widget::widget::*;
use ui::component::game::GamePrivate;
use ui::component::puzzle::puzzle_cell::{PuzzleCell, PuzzleAction};
use resources::thing::ThingImages;
use error::*;

const FIELD_OFFSET_X: u16 =    12;
const FIELD_OFFSET_Y: u16 =    68;
const FIELD_GAP_X: u16 =        4;
const FIELD_GAP_Y: u16 =        4;
const FIELD_TILE_WIDTH: u16 =  48;
const FIELD_TILE_HEIGHT: u16 = 48;

pub struct Puzzle {
    cells: Vec<PuzzleCell>,
}

impl Widget<PuzzleAction> for Puzzle {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        Rect::new(
            FIELD_OFFSET_X as i32,
            FIELD_OFFSET_Y as i32,
            (6 * FIELD_TILE_WIDTH + 5 * FIELD_GAP_X) as u32,
            (6 * FIELD_TILE_HEIGHT + 5 * FIELD_GAP_Y) as u32
        )
    }

    fn on_event(&self, event: &Event) -> EventReaction<PuzzleAction> {
        for child in &self.cells {
            let event2 = event.relative(child.get_rect());
            let reaction = child.on_event(&event2);
            match reaction {
                EventReaction::Redraw => return Ok((reaction, action)),
                EventReaction::StopPropagation => return Ok((reaction, action)),
                EventReaction::NoOp => {},
            }
        }
        EventReaction::NoOp
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for child in &self.cells {
            let c = context.relative(child.get_rect());
            child.draw(&c)?;
        }
        Ok(())
    }
}

pub fn new_puzzle_widget(state: Rc<RefCell<GamePrivate>>) -> Result<Puzzle> {
    let thing_images = ThingImages::new()?;

    let mut container = Puzzle { cells: Vec::new() };

    for row in 0..PUZZLE_SIZE {
        for col in 0..PUZZLE_SIZE {
            let cell_x = (col as i32) * ((FIELD_TILE_WIDTH + FIELD_GAP_X) as i32);
            let cell_y = (row as i32) * ((FIELD_TILE_HEIGHT + FIELD_GAP_Y) as i32);

            let cell = PuzzleCell::new(cell_x, cell_y, state.clone(), row as u8, col as u8, thing_images.clone())?;
            container.cells.push(cell);
        }
    }

    Ok(container)
}
