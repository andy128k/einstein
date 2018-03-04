use rules::{Rule, Thing};
use sdl::video::Surface;
use sdl::video::ll::{SDL_RWops, SDL_LoadBMP_RW, SDL_DisplayFormat, SDL_FreeSurface};
use thing::LARGE_IMAGES;

extern "C" {
    fn SDL_RWFromConstMem(mem: * const ::libc::c_void, size: ::libc::c_int) -> * mut SDL_RWops;
    fn SDL_FreeRW(area: *mut SDL_RWops);
}

fn load_image(data: &[u8]) -> Surface {
    unsafe {
        let op = SDL_RWFromConstMem(data.as_ptr() as * const ::libc::c_void, data.len() as i32);
        let s = SDL_LoadBMP_RW(op, 0);
        SDL_FreeRW(op);
        let screenS = SDL_DisplayFormat(s);
        SDL_FreeSurface(s);
        Surface { raw: screenS, owned: true }
    }
}

fn load_thing_icon(thing: &Thing) -> Surface {
    load_image(LARGE_IMAGES[thing.row as usize][thing.value as usize])
}

pub fn draw_rule(rule: &Rule, surface: &Surface, x: i16, y: i16, h: bool) {
    match *rule {
        Rule::Near(ref thing1, ref thing2) => {
            let icon1 = load_thing_icon(thing1);
            let icon2 = load_thing_icon(thing2);
            surface.blit_at(&icon1, x, y);
            // draw(surface, x + icon1->h, y, iconSet.getNearHintIcon(h));
            surface.blit_at(&icon2, x + icon1.get_width() as i16 * 2, y);
        },
        _ => {}
    }
}

/*
void DirectionRule::draw(Screen *screen, int x, int y, IconSet &iconSet, bool h)
{
    SDL_Surface *icon = iconSet.getLargeIcon(row1, thing1, h);
    screen->draw(x, y, icon);
    screen->draw(x + icon->h, y, iconSet.getSideHintIcon(h));
    screen->draw(x + icon->h*2, y, iconSet.getLargeIcon(row2, thing2, h));
}

void UnderRule::draw(Screen *screen, int x, int y, IconSet &iconSet, bool h)
{
    SDL_Surface *icon = iconSet.getLargeIcon(row1, thing1, h);
    screen->draw(x, y, icon);
    screen->draw(x, y + icon->h, iconSet.getLargeIcon(row2, thing2, h));
}

void BetweenRule::draw(Screen *screen, int x, int y, IconSet &iconSet, bool h)
{
    SDL_Surface *icon = iconSet.getLargeIcon(row1, thing1, h);
    screen->draw(x, y, icon);
    screen->draw(x + icon->w, y, iconSet.getLargeIcon(centerRow, centerThing, h));
    screen->draw(x + icon->w*2, y, iconSet.getLargeIcon(row2, thing2, h));
    SDL_Surface *arrow = iconSet.getBetweenArrow(h);
    screen->draw(x + icon->w - (arrow->w - icon->w) / 2, y + 0, arrow);
}

*/
