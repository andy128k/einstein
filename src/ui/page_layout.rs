use itertools::join;
use regex::Regex;
use sdl::video::Surface;
use sdl2::ttf::Font;
use util::group_by_weight::group_by_weight;
use ui::utils::load_image;
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

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

fn wrap_lines(text: &str, page_width: u16, font: &Font) -> Vec<String> {
    group_by_weight(SPLIT_REGEX.split(text), |words, word| {
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

    fn add_item(&mut self, span: u16, item: PageItem) {
        let height = item.get_height();

        let y = if self.y == 0 {
            0
        } else {
            self.y + span
        };

        if y + height > self.page_height {
            let mut page = Page::new();
            page.push(item.set_y(0));
            self.pages.push(page);
            self.y = height;
        } else {
            self.pages.last_mut().unwrap().push(item.set_y(y));
            self.y = y + height;
        }
    }

    pub fn add_text(&mut self, text: &str, font: &Font) -> Result<()> {
        let lines = wrap_lines(text, self.page_width, font);
        let mut span = 16;
        for line in lines {
            let (width, height) = font.size_of(&line)?;
            self.add_item(span, PageItem::Text(line, 0, 0, width as u16, height as u16));
            span = 4;
        }
        Ok(())
    }

    pub fn add_image(&mut self, content: &[u8]) -> Result<()> {
        let image: Surface = load_image(content)?;
        let width = image.get_width();
        let height = image.get_height();
        let x = (self.page_width - width) / 2;
        self.add_item(16, PageItem::Image(image, x, 0, width, height));
        Ok(())
    }

    pub fn build(self) -> Vec<Page> {
        self.pages
    }
}
