use failure::err_msg;
use rules::{Rule, Thing};
use sdl::video::{Surface, SurfaceFlag, Color};
use sdl::video::ll::{SDL_LoadBMP_RW, SDL_RWFromConstMem};
use super::thing::{LARGE_IMAGES, SMALL_IMAGES};
use error::*;

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");
const HINT_BETWEEN_ICON: &[u8] = include_bytes!("./betwarr.bmp");

fn load_image(data: &[u8]) -> Result<Surface> {
    let surface = unsafe {
        let op = SDL_RWFromConstMem(data.as_ptr() as * const ::libc::c_void, data.len() as i32);
        Surface { raw: SDL_LoadBMP_RW(op, 0), owned: true }
    };
    let surface = surface.display_format().map_err(err_msg)?;
    surface.set_color_key(&[SurfaceFlag::SrcColorKey], Color::RGBA(255, 255, 255, 255));
    Ok(surface)
}

fn load_thing_icon(thing: &Thing) -> Result<Surface> {
    load_image(LARGE_IMAGES[thing.row as usize][thing.value as usize])
}

pub fn draw_thing(thing: &Thing, surface: &Surface, x: i16, y: i16, hightlighted: bool) -> Result<()> {
    let icon = load_image(LARGE_IMAGES[thing.row as usize][thing.value as usize])?;
    surface.blit_at(&icon, x, y);
    Ok(())
}

pub fn draw_small_thing(thing: &Thing, surface: &Surface, x: i16, y: i16, hightlighted: bool) -> Result<()> {
    let icon = load_image(SMALL_IMAGES[thing.row as usize][thing.value as usize])?;
    surface.blit_at(&icon, x, y);
    Ok(())
}

pub fn draw_rule(rule: &Rule, surface: &Surface, x: i16, y: i16, hightlighted: bool) -> Result<()> {
    match *rule {
        Rule::Near(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1)?;
            let hint = load_image(HINT_NEAR_ICON)?;
            let icon2 = load_thing_icon(thing2)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&hint, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        Rule::Direction(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1)?;
            let hint = load_image(HINT_SIDE_ICON)?;
            let icon2 = load_thing_icon(thing2)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&hint, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        Rule::Under(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1)?;
            let icon2 = load_thing_icon(thing2)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&icon2, x, y + icon1.get_height() as i16);
        },
        Rule::Between(ref thing1, ref thing2, ref thing3) => {
            let icon1 = load_thing_icon(thing1)?;
            let icon2 = load_thing_icon(thing2)?;
            let icon3 = load_thing_icon(thing3)?;
            let arrow = load_image(HINT_BETWEEN_ICON)?;
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon3, x + icon1.get_width() as i16 * 2, y);
            surface.blit_at(&arrow, x + ((3 * icon1.get_width() - arrow.get_width()) / 2) as i16, y);
        },
        _ => {}
    }
    Ok(())
}
