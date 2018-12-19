use crate::rules::{Rule, Thing};
use crate::resources::thing::{get_thing_rect, LARGE_THINGS_ATLAS, EMPTY_TILE};
use crate::resources::manager::Resource;
use crate::ui::widget::common::*;
use crate::ui::brick::*;

pub const TILE_WIDTH: u32 = 48;
pub const TILE_HEIGHT: u32 = 48;

const HINT_NEAR_ICON: Resource = resource!("./hint-near.bmp");
const HINT_SIDE_ICON: Resource = resource!("./hint-side.bmp");
const HINT_BETWEEN_ICON: Resource = resource!("./betwarr.bmp");

fn draw_thing(thing: Thing, highlighted: bool) -> Brick {
    Brick::new(TILE_WIDTH, TILE_HEIGHT)
        .background(Background::Image(&LARGE_THINGS_ATLAS, highlighted, Some(get_thing_rect(thing))))
}

pub fn draw_rule(rule: &Rule, highlighted: bool) -> Brick {
    match *rule {
        Rule::Near(thing1, thing2) => {
            Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
                .background(Background::Image(&EMPTY_TILE, highlighted, None))
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(TILE_WIDTH, 0, Brick::new(TILE_WIDTH, TILE_HEIGHT).background(Background::Image(&HINT_NEAR_ICON, highlighted, None)))
                .add(TILE_WIDTH * 2, 0, draw_thing(thing2, highlighted))
        },
        Rule::Direction(thing1, thing2) => {
            Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
                .background(Background::Image(&EMPTY_TILE, highlighted, None))
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(TILE_WIDTH * 2, 0, draw_thing(thing2, highlighted))
                .add(TILE_WIDTH, 0, Brick::new(TILE_WIDTH, TILE_HEIGHT).background(Background::Image(&HINT_SIDE_ICON, highlighted, None)))
        },
        Rule::Under(thing1, thing2) => {
            Brick::new(TILE_WIDTH, TILE_HEIGHT * 2)
                .background(Background::Image(&EMPTY_TILE, highlighted, None))
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(0, TILE_HEIGHT, draw_thing(thing2, highlighted))
        },
        Rule::Between(thing1, thing2, thing3) => {
            Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(TILE_WIDTH, 0, draw_thing(thing2, highlighted))
                .add(TILE_WIDTH * 2, 0, draw_thing(thing3, highlighted))
                .add((3 * TILE_WIDTH - 70) / 2, 0, Brick::new(70, 15).background(Background::Image(&HINT_BETWEEN_ICON, highlighted, None)))
        },
        _ => {
            Brick::new(0, 0)
        }
    }
}
