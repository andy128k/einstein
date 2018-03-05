use failure::err_msg;
use itertools::join;
use sdl::video::{Surface, SurfaceFlag, Color};
use sdl::video::ll::{SDL_LoadBMP_RW, SDL_RWFromConstMem};
use sdl2_ttf::Font;
use util::group_by_weight::group_by_weight;
use error::*;

pub type Page = Vec<PageItem>;

pub enum PageItem {
    Text(String, u16, u16, u16, u16),
    Image(Surface, u16, u16, u16, u16),
}

impl PageItem {
    fn get_height(&self) -> u16 {
        match *self {
            PageItem::Text(_, _, _, _, h) |
            PageItem::Image(_, _, _, _, h) => h
        }
    }

    fn set_y(mut self, new_y: u16) -> Self {
        match self {
            PageItem::Text(_, _, ref mut y, _, _) |
            PageItem::Image(_, _, ref mut y, _, _) => *y = new_y
        }
        self
    }
}

fn load_image(data: &[u8]) -> Result<Surface> {
    let surface = unsafe {
        let op = SDL_RWFromConstMem(data.as_ptr() as * const ::libc::c_void, data.len() as i32);
        Surface { raw: SDL_LoadBMP_RW(op, 0), owned: true }
    };
    let surface = surface.display_format().map_err(err_msg)?;
    Ok(surface)
}

fn wrap_lines(text: &str, page_width: u16, font: &Font) -> Vec<String> {
    group_by_weight(text.split(" "), |words, word| { // TODO regex split \s
        let line = join(words, " ") + " " + word;
        match font.size_of(&line) {
            Ok((width, _)) => (width as u16) < page_width,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                false
            }
        }
    })
    .into_iter()
    .map(|words| join(words, " "))
    .collect()
}

pub struct PagesBuilder {
    page_width: u16,
    page_height: u16,
    pages: Vec<Page>,
    y: u16,
}

impl PagesBuilder {
    pub fn new(page_width: u16, page_height: u16) -> Self {
        Self {
            page_width,
            page_height,
            pages: vec![Page::new()],
            y: 0
        }
    }

    fn add_item(&mut self, item: PageItem) {
        let height = item.get_height();

        if self.y + 10 + height > self.page_height {
            let mut page = Page::new();
            page.push(item.set_y(0));
            self.pages.push(page);
            self.y = height;
        } else {
            self.pages.last_mut().unwrap().push(item.set_y(self.y + 10));
            self.y += 10 + height;
        }
    }

    pub fn add_text(&mut self, text: &str, font: &Font) -> Result<()> {
        let lines = wrap_lines(text, self.page_width, font);
        for line in lines {
            let (width, height) = font.size_of(&line)?;
            self.add_item(PageItem::Text(line, 0, 0, width as u16, height as u16));
        }
        Ok(())
    }

    pub fn add_image(&mut self, content: &[u8]) -> Result<()> {
        let image: Surface = load_image(content)?;
        let width = image.get_width();
        let height = image.get_height();
        let x = (self.page_width - width) / 2;
        self.add_item(PageItem::Image(image, x, 0, width, height));
        Ok(())
    }

    pub fn build(self) -> Vec<Page> {
        self.pages
    }
}
