use std::rc::Rc;
use cell::RefCell;
use sdl::event::Key;
use error::*;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::BackgroundPattern;
use ui::widget::dialog_button::*;
use ui::widget::window::*;
use ui::widget::title::Title;
use ui::widget::page_view::*;
use ui::widget::modal::Modal;
use ui::page_layout::{Page, PagesBuilder};
use resources::fonts::*;
use resources::rules::{get_rules, TextItem};
use resources::messages::Messages;

const WIDTH: u16 = 600;
const HEIGHT: u16 = 500;
const CLIENT_WIDTH: u16 = 570;
const CLIENT_HEIGHT: u16 = 390;

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
    pages: Vec<Rc<Page>>,
    current_page_index: usize,
    current_page: Rc<RefCell<Rc<Page>>>
}

impl DescriptionPrivate {
    fn new(messages: &Messages, text: &[TextItem]) -> Result<Modal<()>> {
        let pages: Vec<Rc<Page>> = make_pages(text, CLIENT_WIDTH, CLIENT_HEIGHT)?
            .into_iter().map(Rc::new).collect();

        let rect = Rect::new(100, 50, WIDTH as u32, HEIGHT as u32);
        let bg = BackgroundPattern::Blue;

        let current_page = Rc::new(RefCell::new(pages[0].clone()));

        let state = Rc::new(RefCell::new(DescriptionPrivate {
            pages,
            current_page_index: 0,
            current_page: current_page.clone()
        }));

        let container = Modal::<()>::new(rect)
            .add(WidgetMapAction::no_action(
                Window::new(Rect::new0(WIDTH as u32, HEIGHT as u32), bg)
            ))
            .add(WidgetMapAction::no_action(
                Title {
                    text: messages.rules.to_string(),
                    rect: Rect::new(150, 10, 300, 40),
                }
            ))
            .add(WidgetMapAction::no_action(
                PageView::new(Rect::new(15, 50, CLIENT_WIDTH as u32, CLIENT_HEIGHT as u32), current_page)
            ))
            .add({
                let state2 = state.clone();
                WidgetMapAction::new(
                    DialogButton::new(Rect::new(10, 465, 80, 25), bg, messages.prev, None, ()),
                    move |_| {
                        state2.borrow_mut().prev();
                        Ok(EventReaction::empty())
                    }
                )
            })
            .add({
                let state2 = state.clone();
                WidgetMapAction::new(
                    DialogButton::new(Rect::new(100, 465, 80, 25), bg, messages.next, None, ()),
                    move |_| {
                        state2.borrow_mut().next();
                        Ok(EventReaction::empty())
                    }
                )
            })
            .add(
                DialogButton::new(Rect::new(510, 465, 80, 25), bg, messages.close, Some(Key::Escape), ())
            );

        Ok(container)
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

pub fn new_help_dialog(messages: &Messages) -> Result<Modal<()>> {
    DescriptionPrivate::new(messages, get_rules())
}
