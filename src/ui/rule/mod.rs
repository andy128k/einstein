use rules::{Rule, Thing};
use sdl::video::Surface;
use sdl::video::ll::{SDL_LoadBMP_RW, SDL_RWFromConstMem};
use super::thing::LARGE_IMAGES;

const HINT_NEAR_ICON: &[u8] = include_bytes!("./hint-near.bmp");
const HINT_SIDE_ICON: &[u8] = include_bytes!("./hint-side.bmp");

fn load_image(data: &[u8]) -> Surface {
    unsafe {
        let op = SDL_RWFromConstMem(data.as_ptr() as * const ::libc::c_void, data.len() as i32);
        let surface = Surface { raw: SDL_LoadBMP_RW(op, 0), owned: true };
        surface.display_format().unwrap()
    }
}

fn load_thing_icon(thing: &Thing) -> Surface {
    load_image(LARGE_IMAGES[thing.row as usize][thing.value as usize])
}

pub fn draw_rule(rule: &Rule, surface: &Surface, x: i16, y: i16, hightlighted: bool) {
    match *rule {
        Rule::Near(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1);
            let hint = load_image(HINT_NEAR_ICON);
            let icon2 = load_thing_icon(thing2);
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&hint, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        Rule::Direction(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1);
            let hint = load_image(HINT_SIDE_ICON);
            let icon2 = load_thing_icon(thing2);
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&hint, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        Rule::Under(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1);
            let icon2 = load_thing_icon(thing2);
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&icon2, x, y + icon1.get_height() as i16);
        },
        Rule::Between(ref thing1, ref thing2, ref thing3) => {
            let icon1 = load_thing_icon(thing1);
            let icon2 = load_thing_icon(thing2);
            let icon3 = load_thing_icon(thing3);
            surface.blit_at(&icon1, x, y);
            surface.blit_at(&icon2, x + icon1.get_width() as i16, y);
            surface.blit_at(&icon3, x + icon1.get_width() as i16 * 2, y);
        },
        _ => {}
    }
}
