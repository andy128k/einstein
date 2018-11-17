use std::rc::Rc;
use cell::RefCell;
use rules::{PUZZLE_SIZE};
use ui::context::Rect;
use ui::widget::widget::*;
use ui::brick::*;
use ui::component::game::GamePrivate;
use ui::component::puzzle::puzzle_cell::{PuzzleCell, PuzzleAction};
use resources::manager::ResourceManager;
use audio::Audio;
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

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<PuzzleAction> {
        let mut reaction = EventReaction::empty();
        for child in self.cells.iter_mut() {
            let event2 = event.relative(child.get_rect());
            let cell_reaction = child.on_event(&event2, resource_manager, audio)?;
            // TODO -relative
            reaction.add(&cell_reaction);
            if reaction.terminated {
                break;
            }
        }
        Ok(reaction)
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_rect().width(), self.get_rect().height());
        for child in &self.cells {
            let cb = child.draw(resource_manager);
            brick.push(child.get_rect().left() as u32, child.get_rect().top() as u32, cb);
        }
        brick
    }
}

pub fn new_puzzle_widget(state: Rc<RefCell<GamePrivate>>) -> Result<Puzzle> {
    let mut container = Puzzle { cells: Vec::new() };

    for row in 0..PUZZLE_SIZE {
        for col in 0..PUZZLE_SIZE {
            let cell_x = (col as i32) * ((FIELD_TILE_WIDTH + FIELD_GAP_X) as i32);
            let cell_y = (row as i32) * ((FIELD_TILE_HEIGHT + FIELD_GAP_Y) as i32);

            let cell = PuzzleCell::new(cell_x, cell_y, state.clone(), row as u8, col as u8)?;
            container.cells.push(cell);
        }
    }

    Ok(container)
}
