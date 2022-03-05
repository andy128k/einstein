use crate::cell::RefCell;
use crate::error::Result;
use crate::resources::manager::ResourceManager;
use crate::resources::rules::TextItem;
use crate::ui::brick::*;
use crate::ui::common::{HorizontalAlign, Size};
use crate::ui::page_layout::*;
use crate::ui::widget::common::*;
use crate::ui::widget::widget::*;
use never::Never;
use sdl2::pixels::Color;
use std::rc::Rc;

pub struct PageViewState {
    text: &'static [TextItem<'static>],
    pages: Vec<Page>,
    current_page_index: usize,
}

impl PageViewState {
    pub fn new(text: &'static [TextItem<'static>]) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            text,
            pages: vec![],
            current_page_index: 0,
        }))
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

    fn make_pages(
        &mut self,
        page_width: u32,
        page_height: u32,
        resource_manager: &dyn ResourceManager,
    ) -> Result<()> {
        if self.pages.is_empty() {
            let mut pages = PagesBuilder::new(page_width as u16, page_height as u16);
            for text_item in self.text {
                match *text_item {
                    TextItem::Text(content) => {
                        pages.add_text(content, &*resource_manager.font(FontSize::TEXT.0))?
                    }
                    TextItem::Image(ref image) => pages.add_image(image, resource_manager)?,
                }
            }
            self.pages = pages.build();
        }
        Ok(())
    }
}

pub struct PageView {
    size: Size,
    state: Rc<RefCell<PageViewState>>,
}

impl PageView {
    pub fn new(size: Size, state: &Rc<RefCell<PageViewState>>) -> Self {
        Self {
            size,
            state: state.clone(),
        }
    }
}

impl Widget<Never> for PageView {
    fn get_size(&self) -> Size {
        self.size
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        self.state
            .borrow_mut()
            .make_pages(
                self.get_size().width,
                self.get_size().height,
                resource_manager,
            )
            .unwrap();

        let mut brick = Brick::new(self.get_size().width, self.get_size().height);
        if let Some(items) = self.state.borrow().current_page() {
            for item in items {
                match *item {
                    PageItem::Text(ref text, x, y, w, h) => {
                        brick.push(
                            x as u32,
                            y as u32,
                            Brick::new(w as u32, h as u32).text(
                                Text::new(&text)
                                    .font_size(FontSize::TEXT)
                                    .color(Color::RGB(255, 255, 255))
                                    .shadow()
                                    .halign(HorizontalAlign::Left),
                            ),
                        );
                    }
                    PageItem::Image(image, x, y, w, h) => {
                        brick.push(
                            x as u32,
                            y as u32,
                            Brick::new(w as u32, h as u32)
                                .background(Background::Image(image, None)),
                        );
                    }
                }
            }
        }
        brick
    }
}
