use std::rc::Rc;
use rules::Thing;
use sdl::video::Surface;
use ui::utils::{load_image, adjust_brightness};
use ui::context::{Rect, Sprite};
use error::*;

pub struct ThingImages {
    large_things: Surface,
    large_things_highlighted: Surface,
    small_things: Surface,
    small_things_highlighted: Surface,
}

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

pub const LARGE_THINGS_ATLAS: &[u8] = include_bytes!("./large.bmp");
pub const SMALL_THINGS_ATLAS: &[u8] = include_bytes!("./small.bmp");

impl ThingImages {
    pub fn new() -> Result<Rc<Self>> {
        let large_things = load_image(LARGE_THINGS_ATLAS)?;
        let large_things_highlighted = adjust_brightness(&large_things, 1.5);
        let small_things = load_image(SMALL_THINGS_ATLAS)?;
        let small_things_highlighted = adjust_brightness(&small_things, 1.5);

        Ok(Rc::new(ThingImages {
            large_things,
            large_things_highlighted,
            small_things,
            small_things_highlighted,
        }))
    }

    pub fn get_thing_image(&self, thing: Thing, highlighted: bool) -> Sprite {
        let rect = get_thing_rect(thing);
        if highlighted {
            Sprite { image: &self.large_things_highlighted, rect }
        } else {
            Sprite { image: &self.large_things, rect }
        }
    }

    pub fn get_small_thing_image(&self, thing: Thing, highlighted: bool) -> Sprite {
        let rect = get_small_thing_rect(thing);
        if highlighted {
            Sprite { image: &self.small_things_highlighted, rect }
        } else {
            Sprite { image: &self.small_things, rect }
        }
    }
}
