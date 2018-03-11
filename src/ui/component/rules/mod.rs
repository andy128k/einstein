use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use failure::err_msg;
use sdl;
use sdl::video::{Surface};
use sdl::event::{Key, Mouse};
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::ttf::Font;
use error::*;
use ui::widget::widget::*;
use ui::widget::label::*;
use ui::widget::button::*;
use ui::widget::window::*;
use ui::widget::container::*;
use ui::widget::page_view::*;
use ui::utils::{load_image, tiled_image, adjust_brightness, adjust_brightness_pixel, draw_bevel, draw_text, HorizontalAlign, VerticalAlign};
use ui::fonts::*;
use ui::main_loop::main_loop;
use ui::page_layout::{Page, PagesBuilder};
use ui::background::BLUE_PATTERN;
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

enum TextItem {
    Text(&'static str),
    Image(&'static [u8]),
}

const RULES_EN: &[TextItem] = &[
    TextItem::Text("The game goal is to open all cards in square of 6x6 cards.
        When every card is open, field looks like this:"),
    TextItem::Image(include_bytes!("./opensquare.bmp")),
    TextItem::Text("Every row of square contains cards of one type only.  For example,
        first row contains arabic digits, second - letters, third - rome digits,
        fouths - dices, fifth - geometric figures, sixs - mathematic symbols."),
    TextItem::Text("Use logic and open cards with method of exclusion.  If card doesn't
        opened, cell contains every possible cards. For example,"),
    TextItem::Image(include_bytes!("./closed.bmp")),
    TextItem::Text("means that this cell may contain every rome digit with exception of 
        III (because card with III image is absent).  To open card click on
        small image with left mouse button.  To exclude card click with right
        mouse button."),
    TextItem::Text("Use tips to solve this puzzle.  There is two types of tips:
        horizontal and vertical.  Vertical tips located at screen bottom.
        For example, vertical tip"),
    TextItem::Image(include_bytes!("./verthint.bmp")),
    TextItem::Text("means that letter 'B' and '+' sign located in the same column."),
    TextItem::Text("Horizontal tips located at the right side of the puzzle square.
        There is few type of horizontal tips.  First type of horizontal
        tip says that two cards located at neighbour columns, but it is
        unknown, which one is at the right side and thich is at the left:"),
    TextItem::Image(include_bytes!("./hornearhint.bmp")),
    TextItem::Text("Second tip type means that one cards is at the left of another.
        It says nothing about distance between that cards.  They may be
        neighbour columns or at the opposite sides of puzzle field:"),
    TextItem::Image(include_bytes!("./horposhint.bmp")),
    TextItem::Text("The last type of tip means that one card is located between
        two another cards:"),
    TextItem::Image(include_bytes!("./horbetweenhint.bmp")),
    TextItem::Text("All three cards must be located in neighbour columns, central
        card is always between other two, but it is unknown, which card is located
        at the right side and which at the left."),
    TextItem::Text("If you no longer need some tip, remove it by right mouse button click.
        You can always see removed tips by pressing 'Switch' button."),
];

const RULES_RU: &[TextItem] = &[
    TextItem::Text("Правила игры очень простые: надо открыть все фишки в квадрате 
        6x6 фишек. После того как все фишки будут открыты, 
        квадрат будет выглядить следующим образом:"),
    TextItem::Image(include_bytes!("./opensquare.bmp")),
    TextItem::Text("В каждой строке квадрата находятся фишки одного типа. Например, 
        в первой строке квадрата находятся арабские цифры,
        во второй - латинские буквы, в третьей - римские цифры,
        в четвертой - игральные кости, в пятой - геометрические фигуры,
        в шестой - математические символы."),
    TextItem::Text("Открывать фишки надо методом исключения. Когда фишка не 
        открыта на ее месте показываются все возможные варианты.
        Например, изображение"),
    TextItem::Image(include_bytes!("./closed.bmp")),
    TextItem::Text("обозначает что в данном месте могут находится любые римские 
        цифры кроме III (квадратик с изображением III отсутствует).
        Чтобы открыть фишку надо нажать на ее уменьшенное изображение 
        левой кнопкой мыши, чтобы исключить фишку - нажмите на ней 
        правой кнопкой мыши."),
    TextItem::Text("Для того, чтобы решить головоломку нужно использовать подсказки.
        Подсказки бывают двух типов: вертикальные и горизонтальные.
        Вертикальные подсказки находятся внизу экрана и выглядят так:"),
    TextItem::Image(include_bytes!("./verthint.bmp")),
    TextItem::Text("Такая подсказка обозначает что буква 'B' и знак '+' находятся 
        в одной колонке, при этом не важно, какой из этих символов находится
        выше, а какой - ниже."),
    TextItem::Text("Горизонтальные подсказки расположены в правой части экрана. 
        Они делятся на несколько типов. Самая простая подсказка говорит 
        о том что две фишки находятся в соседних колонках, при этом не 
        известно, какая из фишек находится левее а какая правее:"),
    TextItem::Image(include_bytes!("./hornearhint.bmp")),
    TextItem::Text("Подсказка следующего типа говорит о том что одна фишка 
        находится в колонке левее другой. Эта подсказка ничего 
        не говорит о том на каком расстоянии друг от друга находятся фишки. 
        Они могут оказаться как в соседних колонках так и находится 
        на значительном расстоянии друг от друга:"),
    TextItem::Image(include_bytes!("./horposhint.bmp")),
    TextItem::Text("Последний тип подсказки указывает что одна фишка находится между 
        двумя другими:"),
    TextItem::Image(include_bytes!("./horbetweenhint.bmp")),
    TextItem::Text("Все три фишки всегда находятся в соседних колонках, фишка 
        указанная в центре всегда находится между двумя другими, но какая 
        фишка правее центральной а какая левее - неизвестно."),
    TextItem::Text("Использованные подсказки удобно удалять пользуясь правой кнопкой мыши. 
        Удаленные подсказки можно посмотреть нажав на кнопку 'Скрытые'. 
        Повторное нажатие на эту кнопку снова покажет неудаленные подсказки."),
];


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

type Description = Container<DescriptionPrivate>;

impl DescriptionPrivate {
    fn new(messages: &Messages, text: &[TextItem]) -> Result<Description> {
        let pages: Vec<Rc<Page>> = make_pages(text, CLIENT_WIDTH, CLIENT_HEIGHT)?
            .into_iter().map(Rc::new).collect();

        let rect = Rect::new(100, 50, WIDTH as u32, HEIGHT as u32);

        let current_page = Rc::new(RefCell::new(pages[0].clone()));
        let mut ptr = Description::new(rect, DescriptionPrivate {
            rect,
            pages,
            current_page_index: Cell::new(0),
            current_page: current_page.clone()
        });

        let window = Window::new(rect.clone(), BLUE_PATTERN)?;

        let title = Label {
            font: title_font()?,
            text: messages.rules.to_string(),
            rect: Rect::new(250, 60, 300, 40),
            color: Color::RGB(255, 255, 0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
            shadow: true
        };

        let page_view = PageView::new(Rect::new(START_X as i32, START_Y as i32, CLIENT_WIDTH as u32, CLIENT_HEIGHT as u32), current_page);

        let prev = {
            let this = ptr.weak_private();
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
            let this = ptr.weak_private();
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

        ptr.add(Box::new(window));
        ptr.add(Box::new(title));
        ptr.add(Box::new(page_view));
        ptr.add(Box::new(prev));
        ptr.add(Box::new(next));
        ptr.add(Box::new(close));

        Ok(ptr)
    }

    fn get_page(&self) -> &Page {
        let current_page_index = self.current_page_index.get();
        &self.pages[current_page_index]
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
    let (messages, rules) = if get_language() == Some("ru".to_string()) {
        (&MESSAGES_RU, RULES_RU)
    } else {
        (&MESSAGES_EN, RULES_EN)
    };
    let description = DescriptionPrivate::new(messages, rules)?;
    main_loop(surface, &description)
}

#[no_mangle]
pub extern fn ein_show_description(surface_ptr: * mut sdl::video::ll::SDL_Surface) -> ::libc::c_int {
    let surface = sdl::video::Surface { raw: surface_ptr, owned: false };
    show_description(&surface).unwrap() as i32
}
