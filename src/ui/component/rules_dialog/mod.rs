use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl::video::{Surface};
use sdl::event::Key;
use sdl2::rect::Rect;
use error::*;
use ui::widget::widget::*;
use ui::widget::dialog_button::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::widget::page_view::*;
use ui::widget::dialog::*;
use ui::main_loop::{main_loop, ModalResult};
use ui::page_layout::{Page, PagesBuilder};
use resources::fonts::*;
use resources::background::BLUE_PATTERN;
use resources::rules::{get_rules, TextItem};
use resources::messages::{get_messages, Messages};

const WIDTH: u16 = 600;
const HEIGHT: u16 = 500;
const CLIENT_WIDTH: u16 = 570;
const CLIENT_HEIGHT: u16 = 390;
const START_X: u16 = 115;
const START_Y: u16 = 100;

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
    current_page_index: usize,
    current_page: Rc<RefCell<Rc<Page>>>
}

type DescriptionPtr = Rc<RefCell<DescriptionPrivate>>;

impl DescriptionPrivate {
    fn new(messages: &Messages, text: &[TextItem]) -> Result<WidgetPtr<ModalResult<()>>> {
        let pages: Vec<Rc<Page>> = make_pages(text, CLIENT_WIDTH, CLIENT_HEIGHT)?
            .into_iter().map(Rc::new).collect();

        let rect = Rect::new(100, 50, WIDTH as u32, HEIGHT as u32);

        let current_page = Rc::new(RefCell::new(pages[0].clone()));

        let state = Rc::new(RefCell::new(DescriptionPrivate {
            rect,
            pages,
            current_page_index: 0,
            current_page: current_page.clone()
        }));

        let container: Vec<WidgetPtr<ModalResult<()>>> = vec![
            Box::new(
                InterceptWidget::default()
            ),
            Box::new(WidgetMapAction::no_action(
                Window::new(rect.clone(), BLUE_PATTERN)?
            )),
            Box::new(WidgetMapAction::no_action(
                Title {
                    text: messages.rules.to_string(),
                    rect: Rect::new(250, 60, 300, 40),
                }
            )),
            Box::new(WidgetMapAction::no_action(
                PageView::new(Rect::new(START_X as i32, START_Y as i32, CLIENT_WIDTH as u32, CLIENT_HEIGHT as u32), current_page)
            )),
            Box::new({
                let state2 = state.clone();
                WidgetMapAction::new(
                    new_dialog_button(Rect::new(110, 515, 80, 25), BLUE_PATTERN, messages.prev, None, ())?,
                    move |_| {
                        state2.borrow_mut().prev();
                        EventReaction::Redraw
                    }
                )
            }),
            Box::new({
                let state2 = state.clone();
                WidgetMapAction::new(
                    new_dialog_button(Rect::new(200, 515, 80, 25), BLUE_PATTERN, messages.next, None, ())?,
                    move |_| {
                        state2.borrow_mut().next();
                        EventReaction::Redraw
                    }
                )
            }),
            Box::new(
                new_dialog_button(Rect::new(610, 515, 80, 25), BLUE_PATTERN, messages.close, Some(Key::Escape), ModalResult(()))?
            ),
        ];

        Ok(Box::new(container))
    }

    fn prev(&mut self) {
        if self.current_page_index > 0 {
            self.current_page_index -= 1;
            *self.current_page.borrow_mut() = self.pages[self.current_page_index].clone();
        }
    }

    fn next(&mut self) {
        if self.current_page_index + 1 < self.pages.len() {
            self.current_page_index += 1;
            *self.current_page.borrow_mut() = self.pages[self.current_page_index].clone();
        }
    }
}

pub fn new_help_dialog(messages: &Messages) -> Result<WidgetPtr<ModalResult<()>>> {
    DescriptionPrivate::new(messages, get_rules())
}

pub fn show_description(surface: &Surface) -> Result<bool> {
    let rect = Rect::new(100, 50, WIDTH as u32, HEIGHT as u32);

    let description = DescriptionPrivate::new(get_messages(), get_rules())?;
    let quit = main_loop(surface, rect, &*description)?.is_none();
    Ok(quit)
}
