use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl;
use sdl::video::{Surface};
use sdl::event::Key;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::widget::container::*;
use ui::widget::page_view::*;
use ui::utils::{HorizontalAlign, VerticalAlign};
use ui::main_loop::main_loop;
use ui::page_layout::{Page, PagesBuilder};
use resources::fonts::*;
use resources::background::BLUE_PATTERN;
use resources::rules::get_rules;
use resources::rules::TextItem;
use locale::get_language;

const WIDTH: u16 = 600;
const HEIGHT: u16 = 500;
const CLIENT_WIDTH: u16 = 570;
const CLIENT_HEIGHT: u16 = 390;
const START_X: u16 = 115;
const START_Y: u16 = 100;

struct Messages {
    rules: &'static str,
    prev: &'static str,
    next: &'static str,
    close: &'static str,
}

const MESSAGES_EN: Messages = Messages {
    rules: "Game Rules",
    prev: "Prev",
    next: "Next",
    close: "Close",
};

const MESSAGES_RU: Messages = Messages {
    rules: "Правила игры",
    prev: "Назад",
    next: "Вперед",
    close: "Закрыть",
};

fn make_pages(text: &[TextItem], page_width: u16, page_height: u16) -> Result<Vec<Page>> {
    let font = text_font()?;
    let mut pages = PagesBuilder::new(page_width, page_height);
    for text_item in text {
        match *text_item {
            TextItem::Text(ref content) => pages.add_text(content, font)?,
            TextItem::Image(ref image) => pages.add_image(image)?
        }
    }
    Ok(pages.build())
}

struct DescriptionPrivate {
    rect: Rect,
    pages: Vec<Rc<Page>>,
    current_page_index: Cell<usize>,
    current_page: Rc<RefCell<Rc<Page>>>
}

type DescriptionPtr = Rc<RefCell<DescriptionPrivate>>;

impl DescriptionPrivate {
    fn new(messages: &Messages, text: &[TextItem]) -> Result<Container<DescriptionPtr>> {
        let pages: Vec<Rc<Page>> = make_pages(text, CLIENT_WIDTH, CLIENT_HEIGHT)?
            .into_iter().map(Rc::new).collect();

        let rect = Rect::new(100, 50, WIDTH as u32, HEIGHT as u32);

        let current_page = Rc::new(RefCell::new(pages[0].clone()));

        let state = Rc::new(RefCell::new(DescriptionPrivate {
            rect,
            pages,
            current_page_index: Cell::new(0),
            current_page: current_page.clone()
        }));

        let mut ptr = Container::new(rect, state.clone());

        ptr.add(Box::new(Window::new(rect.clone(), BLUE_PATTERN)?));

        ptr.add(Box::new(Title {
            text: messages.rules.to_string(),
            rect: Rect::new(250, 60, 300, 40),
        }));

        let page_view = PageView::new(Rect::new(START_X as i32, START_Y as i32, CLIENT_WIDTH as u32, CLIENT_HEIGHT as u32), current_page);

        let prev = {
            let this = Rc::downgrade(&state);
            Button::new(Rect::new(110, 515, 80, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, messages.prev,
                None,
                move || {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().prev()
                    } else {
                        None
                    }
                }
            )?
        };

        let next = {
            let this = Rc::downgrade(&state);
            Button::new(Rect::new(200, 515, 80, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, messages.next,
                None,
                move || {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().next()
                    } else {
                        None
                    }
                }
            )?
        };

        let close = Button::new(Rect::new(610, 515, 80, 25), Color::RGB(255, 255, 0), BLUE_PATTERN, messages.close,
            Some(Key::Escape),
            || Some(Effect::Terminate)
        )?;

        ptr.add(Box::new(page_view));
        ptr.add(Box::new(prev));
        ptr.add(Box::new(next));
        ptr.add(Box::new(close));

        Ok(ptr)
    }

    fn prev(&mut self) -> Option<Effect> {
        let mut current_page_index = self.current_page_index.get();
        if current_page_index > 0 {
            current_page_index -= 1;
            self.current_page_index.set(current_page_index);
            *self.current_page.borrow_mut() = self.pages[current_page_index].clone();
        }
        Some(Effect::Redraw(vec![self.rect]))
    }

    fn next(&mut self) -> Option<Effect> {
        let mut current_page_index = self.current_page_index.get();
        if current_page_index + 1 < self.pages.len() {
            current_page_index += 1;
            self.current_page_index.set(current_page_index);
            *self.current_page.borrow_mut() = self.pages[current_page_index].clone();
        }
        Some(Effect::Redraw(vec![self.rect]))
    }
}

pub fn show_description(surface: &Surface) -> Result<bool> {
    let messages = if get_language() == Some("ru".to_string()) {
        &MESSAGES_RU
    } else {
        &MESSAGES_EN
    };
    let description = DescriptionPrivate::new(messages, get_rules())?;
    main_loop(surface, &description)
}
