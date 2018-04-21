use rules::{Rule, Thing};
use resources::thing::ThingImages;
use ui::utils::adjust_brightness;
use error::*;
use ui::context::{Context, Rect};
use ui::utils::load_image;

const TILE_WIDTH: i32 = 48;
const TILE_HEIGHT: i32 = 48;

const RECT_1_0: Rect = Rect(TILE_WIDTH * 1, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_2_0: Rect = Rect(TILE_WIDTH * 2, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
const RECT_0_1: Rect = Rect(0, TILE_HEIGHT, TILE_WIDTH as u32, TILE_HEIGHT as u32);

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");
const HINT_BETWEEN_ICON: &[u8] = include_bytes!("./betwarr.bmp");

pub fn draw_thing(context: &Context, images: &ThingImages, thing: Thing) -> Result<()> {
    let icon = images.get_thing_image(thing)?;
    context.image(&icon, 0, 0);
    Ok(())
}

pub fn draw_rule(images: &ThingImages, rule: &Rule, context: &Context, highlighted: bool) -> Result<()> {
    let bg = if highlighted {
        &images.large_bg_highlighted
    } else {
        &images.large_bg
    };

    match *rule {
        Rule::Near(thing1, thing2) => {
            context.image(bg, 0, 0);
            context.image(bg, TILE_WIDTH * 2, 0);

            draw_thing(context, images, thing1)?;
            draw_thing(&context.relative(RECT_2_0), images, thing2)?;

            let mut hint = load_image(HINT_NEAR_ICON)?;
            if highlighted {
                hint = adjust_brightness(&hint, 1.5);
            }
            context.image(&hint, TILE_WIDTH, 0);
        },
        Rule::Direction(thing1, thing2) => {
            context.image(bg, 0, 0);
            context.image(bg, TILE_WIDTH * 2, 0);

            draw_thing(context, images, thing1)?;
            draw_thing(&context.relative(RECT_2_0), images, thing2)?;

            let mut hint = load_image(HINT_SIDE_ICON)?;
            if highlighted {
                hint = adjust_brightness(&hint, 1.5);
            }
            context.image(&hint, TILE_WIDTH, 0);
        },
        Rule::Under(thing1, thing2) => {
            context.image(bg, 0, 0);
            context.image(bg, 0, TILE_HEIGHT);

            draw_thing(context, images, thing1)?;
            draw_thing(&context.relative(RECT_0_1), images, thing2)?;
        },
        Rule::Between(thing1, thing2, thing3) => {
            context.image(bg, 0, 0);
            context.image(bg, TILE_WIDTH, 0);
            context.image(bg, TILE_WIDTH * 2, 0);

            draw_thing(context, images, thing1)?;
            draw_thing(&context.relative(RECT_1_0), images, thing2)?;
            draw_thing(&context.relative(RECT_2_0), images, thing3)?;

            let mut arrow = load_image(HINT_BETWEEN_ICON)?;
            if highlighted {
                arrow = adjust_brightness(&arrow, 1.5);
            }
            context.image(&arrow, (3 * TILE_WIDTH - arrow.get_width() as i32) / 2, 0);
        },
        _ => {}
    }
    Ok(())
}
