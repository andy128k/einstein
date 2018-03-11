use rules::{Rule, Thing};
use sdl::video::{Surface, SurfaceFlag, Color};
use super::thing::{LARGE_IMAGES, SMALL_IMAGES};
use super::utils::adjust_brightness;
use error::*;
use ui::utils::load_image;

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");
const HINT_BETWEEN_ICON: &[u8] = include_bytes!("./betwarr.bmp");

fn load_thing_icon(thing: Thing, hightlighted: bool) -> Result<Surface> {
    let mut image = load_image(LARGE_IMAGES[thing.row as usize][thing.value as usize])?;
    if hightlighted {
        image = adjust_brightness(&image, 1.5, false);
    }
    Ok(image)
}

pub fn draw_thing(thing: Thing, surface: &Surface, x: i16, y: i16, hightlighted: bool) -> Result<()> {
    let mut icon = load_image(LARGE_IMAGES[thing.row as usize][thing.value as usize])?;
    if hightlighted {
        icon = adjust_brightness(&icon, 1.5, false);
    }
    surface.blit_at(&icon, x, y);
    Ok(())
}

pub fn draw_small_thing(thing: Thing, surface: &Surface, x: i16, y: i16, hightlighted: bool) -> Result<()> {
    let mut icon = load_image(SMALL_IMAGES[thing.row as usize][thing.value as usize])?;
    if hightlighted {
        icon = adjust_brightness(&icon, 1.5, false);
    }
    surface.blit_at(&icon, x, y);
    Ok(())
}

pub fn draw_rule(rule: &Rule, surface: &Surface, x: i16, y: i16, hightlighted: bool) -> Result<()> {
    match *rule {
        Rule::Near(thing1, thing2) => {
            let icon1 = load_thing_icon(thing1, hightlighted)?;
            let mut hint = load_image(HINT_NEAR_ICON)?;
            if hightlighted {
                hint = adjust_brightness(&hint, 1.5, false);
            }
            let icon2 = load_thing_icon(thing2, hightlighted)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&hint, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        Rule::Direction(thing1, thing2) => {
            let icon1 = load_thing_icon(thing1, hightlighted)?;
            let mut hint = load_image(HINT_SIDE_ICON)?;
            if hightlighted {
                hint = adjust_brightness(&hint, 1.5, false);
            }
            let icon2 = load_thing_icon(thing2, hightlighted)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&hint, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        Rule::Under(thing1, thing2) => {
            let icon1 = load_thing_icon(thing1, hightlighted)?;
            let icon2 = load_thing_icon(thing2, hightlighted)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&icon2, x, y + icon1.get_height() as i16);
        },
        Rule::Between(thing1, thing2, thing3) => {
            let icon1 = load_thing_icon(thing1, hightlighted)?;
            let icon2 = load_thing_icon(thing2, hightlighted)?;
            let icon3 = load_thing_icon(thing3, hightlighted)?;
            let mut arrow = load_image(HINT_BETWEEN_ICON)?;
            arrow.set_color_key(&[SurfaceFlag::SrcColorKey], Color::RGBA(255, 255, 255, 255));
            if hightlighted {
                arrow = adjust_brightness(&arrow, 1.5, false);
            }
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon3, x + icon1.get_width() as i16 * 2, y);
            surface.blit_at(&arrow, x + ((3 * icon1.get_width() - arrow.get_width()) / 2) as i16, y);
        },
        _ => {}
    }
    Ok(())
}
