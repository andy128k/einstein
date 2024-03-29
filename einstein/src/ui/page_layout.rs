use crate::error::*;
use crate::resources::manager::{Resource, ResourceManager};
use crate::util::group_by_weight::GroupByWeight;
use lazy_static::lazy_static;
use regex::Regex;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;

pub type Page = Vec<PageItem>;

pub enum PageItem {
    Text(String, u16, u16, u16, u16),
    Image(&'static Resource, u16, u16, u16, u16),
}

impl PageItem {
    fn get_height(&self) -> u16 {
        match *self {
            PageItem::Text(_, _, _, _, h) | PageItem::Image(_, _, _, _, h) => h,
        }
    }

    fn set_y(mut self, new_y: u16) -> Self {
        match self {
            PageItem::Text(_, _, ref mut y, _, _) | PageItem::Image(_, _, ref mut y, _, _) => {
                *y = new_y
            }
        }
        self
    }
}

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

fn wrap_lines(text: &str, page_width: u16, font: &Font) -> Vec<String> {
    SPLIT_REGEX
        .split(text)
        .group_by_weight(|words, word| {
            let line = words.join(" ") + " " + word;
            match font.size_of(&line) {
                Ok((width, _)) => width < page_width.into(),
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    false
                }
            }
        })
        .into_iter()
        .map(|words| words.join(" "))
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
            y: 0,
        }
    }

    fn add_item(&mut self, span: u16, item: PageItem) {
        let height = item.get_height();

        let y = if self.y == 0 { 0 } else { self.y + span };

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
            self.add_item(
                span,
                PageItem::Text(line, 0, 0, width as u16, height as u16),
            );
            span = 4;
        }
        Ok(())
    }

    pub fn add_image(
        &mut self,
        image: &'static Resource,
        resource_manager: &dyn ResourceManager,
    ) -> Result<()> {
        let img = resource_manager.image(image);
        let TextureQuery { width, height, .. } = img.query();
        let x = (self.page_width - width as u16) / 2;
        self.add_item(
            16,
            PageItem::Image(image, x, 0, width as u16, height as u16),
        );
        Ok(())
    }

    pub fn build(self) -> Vec<Page> {
        self.pages
    }
}
