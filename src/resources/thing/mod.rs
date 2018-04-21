use std::rc::Rc;
use std::collections::BTreeMap;
use rules::{Thing, PUZZLE_SIZE};
use sdl::video::Surface;
use ui::utils::{load_image, adjust_brightness};
use error::*;

const LARGE_IMAGES: [[&[u8]; 6]; 6] = [
    [
        include_bytes!(concat!("./", "large", "/", "a1.bmp")),
        include_bytes!(concat!("./", "large", "/", "a2.bmp")),
        include_bytes!(concat!("./", "large", "/", "a3.bmp")),
        include_bytes!(concat!("./", "large", "/", "a4.bmp")),
        include_bytes!(concat!("./", "large", "/", "a5.bmp")),
        include_bytes!(concat!("./", "large", "/", "a6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "large", "/", "b1.bmp")),
        include_bytes!(concat!("./", "large", "/", "b2.bmp")),
        include_bytes!(concat!("./", "large", "/", "b3.bmp")),
        include_bytes!(concat!("./", "large", "/", "b4.bmp")),
        include_bytes!(concat!("./", "large", "/", "b5.bmp")),
        include_bytes!(concat!("./", "large", "/", "b6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "large", "/", "c1.bmp")),
        include_bytes!(concat!("./", "large", "/", "c2.bmp")),
        include_bytes!(concat!("./", "large", "/", "c3.bmp")),
        include_bytes!(concat!("./", "large", "/", "c4.bmp")),
        include_bytes!(concat!("./", "large", "/", "c5.bmp")),
        include_bytes!(concat!("./", "large", "/", "c6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "large", "/", "d1.bmp")),
        include_bytes!(concat!("./", "large", "/", "d2.bmp")),
        include_bytes!(concat!("./", "large", "/", "d3.bmp")),
        include_bytes!(concat!("./", "large", "/", "d4.bmp")),
        include_bytes!(concat!("./", "large", "/", "d5.bmp")),
        include_bytes!(concat!("./", "large", "/", "d6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "large", "/", "e1.bmp")),
        include_bytes!(concat!("./", "large", "/", "e2.bmp")),
        include_bytes!(concat!("./", "large", "/", "e3.bmp")),
        include_bytes!(concat!("./", "large", "/", "e4.bmp")),
        include_bytes!(concat!("./", "large", "/", "e5.bmp")),
        include_bytes!(concat!("./", "large", "/", "e6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "large", "/", "f1.bmp")),
        include_bytes!(concat!("./", "large", "/", "f2.bmp")),
        include_bytes!(concat!("./", "large", "/", "f3.bmp")),
        include_bytes!(concat!("./", "large", "/", "f4.bmp")),
        include_bytes!(concat!("./", "large", "/", "f5.bmp")),
        include_bytes!(concat!("./", "large", "/", "f6.bmp")),
    ],
];

const SMALL_IMAGES: [[&[u8]; 6]; 6] = [
    [
        include_bytes!(concat!("./", "small", "/", "a1.bmp")),
        include_bytes!(concat!("./", "small", "/", "a2.bmp")),
        include_bytes!(concat!("./", "small", "/", "a3.bmp")),
        include_bytes!(concat!("./", "small", "/", "a4.bmp")),
        include_bytes!(concat!("./", "small", "/", "a5.bmp")),
        include_bytes!(concat!("./", "small", "/", "a6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "small", "/", "b1.bmp")),
        include_bytes!(concat!("./", "small", "/", "b2.bmp")),
        include_bytes!(concat!("./", "small", "/", "b3.bmp")),
        include_bytes!(concat!("./", "small", "/", "b4.bmp")),
        include_bytes!(concat!("./", "small", "/", "b5.bmp")),
        include_bytes!(concat!("./", "small", "/", "b6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "small", "/", "c1.bmp")),
        include_bytes!(concat!("./", "small", "/", "c2.bmp")),
        include_bytes!(concat!("./", "small", "/", "c3.bmp")),
        include_bytes!(concat!("./", "small", "/", "c4.bmp")),
        include_bytes!(concat!("./", "small", "/", "c5.bmp")),
        include_bytes!(concat!("./", "small", "/", "c6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "small", "/", "d1.bmp")),
        include_bytes!(concat!("./", "small", "/", "d2.bmp")),
        include_bytes!(concat!("./", "small", "/", "d3.bmp")),
        include_bytes!(concat!("./", "small", "/", "d4.bmp")),
        include_bytes!(concat!("./", "small", "/", "d5.bmp")),
        include_bytes!(concat!("./", "small", "/", "d6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "small", "/", "e1.bmp")),
        include_bytes!(concat!("./", "small", "/", "e2.bmp")),
        include_bytes!(concat!("./", "small", "/", "e3.bmp")),
        include_bytes!(concat!("./", "small", "/", "e4.bmp")),
        include_bytes!(concat!("./", "small", "/", "e5.bmp")),
        include_bytes!(concat!("./", "small", "/", "e6.bmp")),
    ],
    [
        include_bytes!(concat!("./", "small", "/", "f1.bmp")),
        include_bytes!(concat!("./", "small", "/", "f2.bmp")),
        include_bytes!(concat!("./", "small", "/", "f3.bmp")),
        include_bytes!(concat!("./", "small", "/", "f4.bmp")),
        include_bytes!(concat!("./", "small", "/", "f5.bmp")),
        include_bytes!(concat!("./", "small", "/", "f6.bmp")),
    ],
];

pub struct ThingImages(BTreeMap<u32, Surface>);

fn make_key(row: u8, value: u8, small: bool, hightlighted: bool) -> u32 {
    let mut key: u32 = ((row as u32) << 8) | (value as u32);
    if small {
        key |= 0x01_00_00u32;
    }
    if hightlighted {
        key |= 0x10_00_00u32;
    }
    key
}

impl ThingImages {
    pub fn new() -> Result<Rc<Self>> {
        let mut things = BTreeMap::new();
        for size in 0..2 {
            let bytes = if size == 0 { LARGE_IMAGES } else { SMALL_IMAGES };
            for row in 0..PUZZLE_SIZE {
                for value in 0..PUZZLE_SIZE {
                    let image = load_image(bytes[row as usize][value as usize])?;
                    let hightlighted = adjust_brightness(&image, 1.5);
                    things.insert(make_key(row as u8, value as u8, size > 0, false), image);
                    things.insert(make_key(row as u8, value as u8, size > 0, true), hightlighted);
                }
            }
        }
        Ok(Rc::new(ThingImages(things)))
    }

    pub fn get_thing_image(&self, thing: Thing, highlighted: bool) -> Result<&Surface> {
        self.0.get(&make_key(thing.row, thing.value, false, highlighted))
            .ok_or_else(|| format_err!("Image for {:?} highlighted={} doesn't exist", thing, highlighted))
    }

    pub fn get_small_thing_image(&self, thing: Thing, highlighted: bool) -> Result<&Surface> {
        self.0.get(&make_key(thing.row, thing.value, true, highlighted))
            .ok_or_else(|| format_err!("Small image for {:?} highlighted={} doesn't exist", thing, highlighted))
    }
}
