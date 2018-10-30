use std::rc::Rc;
use cell::RefCell;
use sdl2::pixels::Color;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::*;
use ui::widget::brick::*;
use ui::page_layout::*;
use resources::manager::ResourceManager;
use resources::rules::TextItem;

pub struct PageViewState {
    text: &'static [TextItem<'static>],
    pages: Vec<Page>,
    current_page_index: usize,
}

impl PageViewState {
    pub fn new(text: &'static [TextItem<'static>]) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { text, pages: vec![], current_page_index: 0 }))
    }

    pub fn current_page(&self) -> Option<&Page> {
        self.pages.get(self.current_page_index)
    }

    pub fn prev(&mut self) {
        if self.current_page_index > 0 {
            self.current_page_index -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.current_page_index + 1 < self.pages.len() {
            self.current_page_index += 1;
        }
    }

    fn make_pages(&mut self, page_width: u32, page_height: u32, resource_manager: &mut ResourceManager) -> Result<(), ::failure::Error> {
        if self.pages.is_empty() {
            let mut pages = PagesBuilder::new(page_width as u16, page_height as u16);
            for text_item in self.text {
                match *text_item {
                    TextItem::Text(ref content) => pages.add_text(content, resource_manager.font(FontSize::TEXT.0))?,
                    TextItem::Image(image_name, image) => pages.add_image(image_name, image, resource_manager)?,
                }
            }
            self.pages = pages.build();
        }
        Ok(())
    }
}

pub struct PageView {
    rect: Rect,
    state: Rc<RefCell<PageViewState>>,
}

impl PageView {
    pub fn new(rect: Rect, state: &Rc<RefCell<PageViewState>>) -> Self {
        Self { rect, state: state.clone() }
    }
}

impl Widget<Nothing> for PageView {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, resource_manager: &mut ResourceManager) -> Brick {
        self.state.borrow_mut().make_pages(self.get_client_rect().width(), self.get_client_rect().height(), resource_manager).unwrap();

        let mut brick = Brick::new(self.get_rect());
        if let Some(items) = self.state.borrow().current_page() {
            for item in items {
                match *item {
                    PageItem::Text(ref text, x, y, w, h) => {
                        brick.push(
                            Brick::new(Rect::new(x as i32, y as i32, w as u32, h as u32))
                                .text(Text::new(&text).font_size(FontSize::TEXT).color(Color::RGB(255, 255, 255)).shadow().halign(HorizontalAlign::Left))
                        );
                    },
                    PageItem::Image(image_name, image_bytes, x, y, w, h) => {
                        brick.push(
                            Brick::new(Rect::new(x as i32, y as i32, w as u32, h as u32))
                                .background(BackgroundPattern::Custom(image_name, image_bytes, false))
                        );
                    }
                }
            }
        }
        brick
    }
}
