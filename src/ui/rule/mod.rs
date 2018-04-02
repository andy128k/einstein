use rules::{Rule};
use sdl::video::{SurfaceFlag, Color};
use resources::thing::ThingImages;
use super::utils::adjust_brightness;
use error::*;
use ui::context::Context;
use ui::utils::load_image;

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");
const HINT_BETWEEN_ICON: &[u8] = include_bytes!("./betwarr.bmp");

pub fn draw_rule(images: &ThingImages, rule: &Rule, context: &Context, hightlighted: bool) -> Result<()> {
    match *rule {
        Rule::Near(thing1, thing2) => {
            let icon1 = images.get_thing_image(thing1, hightlighted)?;
            let mut hint = load_image(HINT_NEAR_ICON)?;
            if hightlighted {
                hint = adjust_brightness(&hint, 1.5);
            }
            let icon2 = images.get_thing_image(thing2, hightlighted)?;
            context.image(&icon1, 0, 0);
            context.image(&hint, icon1.get_width() as i32, 0);
            context.image(&icon2, icon1.get_width() as i32 * 2, 0);
        },
        Rule::Direction(thing1, thing2) => {
            let icon1 = images.get_thing_image(thing1, hightlighted)?;
            let mut hint = load_image(HINT_SIDE_ICON)?;
            if hightlighted {
                hint = adjust_brightness(&hint, 1.5);
            }
            let icon2 = images.get_thing_image(thing2, hightlighted)?;
            context.image(&icon1, 0, 0);
            context.image(&hint, icon1.get_width() as i32, 0);
            context.image(&icon2, icon1.get_width() as i32 * 2, 0);
        },
        Rule::Under(thing1, thing2) => {
            let icon1 = images.get_thing_image(thing1, hightlighted)?;
            let icon2 = images.get_thing_image(thing2, hightlighted)?;
            context.image(&icon1, 0, 0);
            context.image(&icon2, 0, icon1.get_height() as i32);
        },
        Rule::Between(thing1, thing2, thing3) => {
            let icon1 = images.get_thing_image(thing1, hightlighted)?;
            let icon2 = images.get_thing_image(thing2, hightlighted)?;
            let icon3 = images.get_thing_image(thing3, hightlighted)?;
            let mut arrow = load_image(HINT_BETWEEN_ICON)?;
            arrow.set_color_key(&[SurfaceFlag::SrcColorKey], Color::RGBA(255, 255, 255, 255));
            if hightlighted {
                arrow = adjust_brightness(&arrow, 1.5);
            }
            context.image(&icon1, 0, 0);
            context.image(&icon2, icon1.get_width() as i32, 0);
            context.image(&icon3, icon1.get_width() as i32 * 2, 0);
            context.image(&arrow, ((3 * icon1.get_width() - arrow.get_width()) / 2) as i32, 0);
        },
        _ => {}
    }
    Ok(())
}
