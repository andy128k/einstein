use rules::{Rule, Thing};
use resources::manager::ResourceManager;
use resources::thing::{get_thing_rect, LARGE_THINGS_ATLAS};
use error::*;
use ui::context::{Context, Rect, Sprite};

const TILE_WIDTH: i32 = 48;
const TILE_HEIGHT: i32 = 48;

const RECT_1_0: Rect = Rect(TILE_WIDTH * 1, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_2_0: Rect = Rect(TILE_WIDTH * 2, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_0_1: Rect = Rect(0, TILE_HEIGHT, TILE_WIDTH as u32, TILE_HEIGHT as u32);

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");
const HINT_BETWEEN_ICON: &[u8] = include_bytes!("./betwarr.bmp");

fn draw_thing(context: &Context, resource_manager: &mut ResourceManager, thing: Thing, highlighted: bool) {
    let atlas = resource_manager.image_h("LARGE_THINGS_ATLAS", LARGE_THINGS_ATLAS, highlighted);
    let sprite = Sprite { image: atlas, rect: get_thing_rect(thing) };
    context.sprite(&sprite, 0, 0);
}

pub fn draw_rule(context: &Context, resource_manager: &mut ResourceManager, rule: &Rule, highlighted: bool) -> Result<()> {
    match *rule {
        Rule::Near(thing1, thing2) => {
            draw_thing(context, resource_manager, thing1, highlighted);
            draw_thing(&context.relative(RECT_2_0), resource_manager, thing2, highlighted);

            let hint = resource_manager.image_h("HINT_NEAR_ICON", HINT_NEAR_ICON, highlighted);
            context.image(&hint, TILE_WIDTH, 0);
        },
        Rule::Direction(thing1, thing2) => {
            draw_thing(context, resource_manager, thing1, highlighted);
            draw_thing(&context.relative(RECT_2_0), resource_manager, thing2, highlighted);

            let hint = resource_manager.image_h("HINT_SIDE_ICON", HINT_SIDE_ICON, highlighted);
            context.image(&hint, TILE_WIDTH, 0);
        },
        Rule::Under(thing1, thing2) => {
            draw_thing(context, resource_manager, thing1, highlighted);
            draw_thing(&context.relative(RECT_0_1), resource_manager, thing2, highlighted);
        },
        Rule::Between(thing1, thing2, thing3) => {
            draw_thing(context, resource_manager, thing1, highlighted);
            draw_thing(&context.relative(RECT_1_0), resource_manager, thing2, highlighted);
            draw_thing(&context.relative(RECT_2_0), resource_manager, thing3, highlighted);

            let arrow = resource_manager.image_h("HINT_BETWEEN_ICON", HINT_BETWEEN_ICON, highlighted);
            context.image(&arrow, (3 * TILE_WIDTH - arrow.get_width() as i32) / 2, 0);
        },
        _ => {}
    }
    Ok(())
}
