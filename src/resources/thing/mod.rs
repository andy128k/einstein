use rules::Thing;
use ui::context::Rect;

pub const LARGE_THINGS_ATLAS: &[u8] = include_bytes!("./large.bmp");
pub const SMALL_THINGS_ATLAS: &[u8] = include_bytes!("./small.bmp");
pub const EMPTY_TILE: &[u8] = include_bytes!("./tile.bmp");

const LARGE_TILE_WIDTH: i32 = 48;
const LARGE_TILE_HEIGHT: i32 = 48;
const SMALL_TILE_WIDTH: i32 = 16;
const SMALL_TILE_HEIGHT: i32 = 16;

pub fn get_thing_rect(thing: Thing) -> Rect {
    Rect::new(
        thing.value as i32 * LARGE_TILE_WIDTH,
        thing.row as i32 * LARGE_TILE_HEIGHT,
        LARGE_TILE_WIDTH as u32,
        LARGE_TILE_HEIGHT as u32
    )
}

pub fn get_small_thing_rect(thing: Thing) -> Rect {
    Rect::new(
        thing.value as i32 * SMALL_TILE_WIDTH,
        thing.row as i32 * SMALL_TILE_HEIGHT,
        SMALL_TILE_WIDTH as u32,
        SMALL_TILE_HEIGHT as u32
    )
}
