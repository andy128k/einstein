use rules::{Rule, Thing};
use resources::manager::ResourceManager;
use resources::thing::{get_thing_rect, LARGE_THINGS_ATLAS, EMPTY_TILE};
use ui::context::Rect;
use ui::widget::common::*;
use ui::widget::brick::*;

const TILE_WIDTH: i32 = 48;
const TILE_HEIGHT: i32 = 48;

const RECT_0_0: Rect = Rect(0, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_1_0: Rect = Rect(TILE_WIDTH * 1, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_2_0: Rect = Rect(TILE_WIDTH * 2, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_0_1: Rect = Rect(0, TILE_HEIGHT, TILE_WIDTH as u32, TILE_HEIGHT as u32);

const RECT_WIDE: Rect = Rect(0, 0, TILE_WIDTH as u32 * 3, TILE_HEIGHT as u32);
const RECT_TALL: Rect = Rect(0, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32 * 2);

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");
const HINT_BETWEEN_ICON: &[u8] = include_bytes!("./betwarr.bmp");

fn draw_thing(rect: Rect, thing: Thing, highlighted: bool) -> Brick {
    Brick::new(rect)
        .background(BackgroundPattern::Sprite("LARGE_THINGS_ATLAS", LARGE_THINGS_ATLAS, highlighted, get_thing_rect(thing)))
}

pub fn draw_rule(resource_manager: &mut ResourceManager, rule: &Rule, highlighted: bool) -> Brick {
    match *rule {
        Rule::Near(thing1, thing2) => {
            Brick::new(RECT_WIDE)
                .background(BackgroundPattern::Custom("EMPTY_TILE", EMPTY_TILE, highlighted))
                .add(draw_thing(RECT_0_0, thing1, highlighted))
                .add(Brick::new(RECT_1_0).background(BackgroundPattern::Custom("HINT_NEAR_ICON", HINT_NEAR_ICON, highlighted)))
                .add(draw_thing(RECT_2_0, thing2, highlighted))
        },
        Rule::Direction(thing1, thing2) => {
            Brick::new(RECT_WIDE)
                .background(BackgroundPattern::Custom("EMPTY_TILE", EMPTY_TILE, highlighted))
                .add(draw_thing(RECT_0_0, thing1, highlighted))
                .add(draw_thing(RECT_2_0, thing2, highlighted))
                .add(Brick::new(RECT_1_0).background(BackgroundPattern::Custom("HINT_SIDE_ICON", HINT_SIDE_ICON, highlighted)))
        },
        Rule::Under(thing1, thing2) => {
            Brick::new(RECT_TALL)
                .background(BackgroundPattern::Custom("EMPTY_TILE", EMPTY_TILE, highlighted))
                .add(draw_thing(RECT_0_0, thing1, highlighted))
                .add(draw_thing(RECT_0_1, thing2, highlighted))
        },
        Rule::Between(thing1, thing2, thing3) => {
            Brick::new(RECT_WIDE)
                .add(draw_thing(RECT_0_0, thing1, highlighted))
                .add(draw_thing(RECT_1_0, thing2, highlighted))
                .add(draw_thing(RECT_2_0, thing3, highlighted))
                .add(Brick::new(Rect::new((3 * TILE_WIDTH - 70) / 2, 0, 70, 15)).background(BackgroundPattern::Custom("HINT_BETWEEN_ICON", HINT_BETWEEN_ICON, highlighted)))
        },
        _ => {
            Brick::new(Default::default())
        }
    }
}
