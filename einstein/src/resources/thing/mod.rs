use crate::resources::manager::Resource;
use crate::ui::common::Rect;
use einstein_puzzle::rules::Thing;

pub const LARGE_THINGS_ATLAS: Resource = resource!("./large.bmp");
pub const LARGE_THINGS_ATLAS_HIGHLIGHTED: Resource = resource!("./large-h.bmp");

pub const SMALL_THINGS_ATLAS: Resource = resource!("./small.bmp");
pub const SMALL_THINGS_ATLAS_HIGHLIGHTED: Resource = resource!("./small-h.bmp");

pub const EMPTY_TILE: Resource = resource!("./tile.bmp");
pub const EMPTY_TILE_HIGHLIGHTED: Resource = resource!("./tile-h.bmp");

const LARGE_TILE_WIDTH: i32 = 48;
const LARGE_TILE_HEIGHT: i32 = 48;
const SMALL_TILE_WIDTH: i32 = 16;
const SMALL_TILE_HEIGHT: i32 = 16;

pub fn get_thing_rect(thing: Thing) -> Rect {
    Rect::new(
        thing.value.0 as i32 * LARGE_TILE_WIDTH,
        thing.row.0 as i32 * LARGE_TILE_HEIGHT,
        LARGE_TILE_WIDTH as u32,
        LARGE_TILE_HEIGHT as u32,
    )
}

pub fn get_small_thing_rect(thing: Thing) -> Rect {
    Rect::new(
        thing.value.0 as i32 * SMALL_TILE_WIDTH,
        thing.row.0 as i32 * SMALL_TILE_HEIGHT,
        SMALL_TILE_WIDTH as u32,
        SMALL_TILE_HEIGHT as u32,
    )
}
