use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use error::*;
use storage::*;
use ui::widget::widget::*;
use ui::widget::menu_button::*;
use ui::widget::container::*;
use ui::widget::image::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign};
use resources::fonts::*;
use ui::main_loop::main_loop;
use ui::component::game::{game_run, GamePrivate};
use ui::component::load_dialog::load_game;
use ui::component::topscores_dialog::show_scores;
use ui::component::rules_dialog::show_description;
use ui::component::options_dialog::show_options_window;
use ui::component::about_dialog::show_about;

const MENU_BG: &[u8] = include_bytes!("./nova.bmp");

fn make_menu(surface: Rc<Surface>, storage: Rc<RefCell<Storage>>) -> Result<Container<()>> {
    let rect = Rect::new(0, 0, 800, 600);

    let mut container = Container::new(rect, ());

    container.add(Box::new(Image::new(rect, MENU_BG)?));

    // Font font(L"nova.ttf", 28);
    // std::wstring s(msg(L"einsteinFlowix"));
    // int width = font.getWidth(s);
    // font.draw(screen->getSurface(), (screen->getWidth() - width) / 2, 30, 255,255,255, true, s);

    // Font urlFont(L"luximb.ttf", 16);
    // s = L"http://games.flowix.com";
    // width = urlFont.getWidth(s);
    // urlFont.draw(screen->getSurface(), (screen->getWidth() - width) / 2, 60, 255,255,0, true, s);

    container.add(Box::new({
        let surface2 = surface.clone();
        let storage2 = storage.clone();
        new_menu_button(Rect::new(550, 340, 220, 30), "newGame", None,
            move || {
                let game = GamePrivate::new().unwrap();
                let quit = game_run(surface2.clone(), game, storage2.clone()).unwrap();
                if quit {
                    Some(Effect::Quit)
                } else {
                    Some(Effect::Redraw(vec![rect]))
                }
            }
        )
    }));
    container.add(Box::new({
        let surface2 = surface.clone();
        let storage2 = storage.clone();
        new_menu_button(Rect::new(550, 370, 220, 30), "loadGame", None,
            move || {
                let game = load_game(surface2.clone(), &storage2.borrow()).unwrap()?;
                let quit = game_run(surface2.clone(), Rc::new(RefCell::new(game)), storage2.clone()).unwrap();
                if quit {
                    Some(Effect::Quit)
                } else {
                    Some(Effect::Redraw(vec![rect]))
                }
            }
        )
    }));
    container.add(Box::new({
        let surface2 = surface.clone();
        let storage2 = storage.clone();
        new_menu_button(Rect::new(550, 400, 220, 30), "topScores", None,
            move || {
                let quit = show_scores(&surface2, &storage2.borrow().scores, None).unwrap();
                if quit {
                    Some(Effect::Quit)
                } else {
                    Some(Effect::Redraw(vec![rect]))
                }
            }
        )
    }));
    container.add(Box::new({
        let surface2 = surface.clone();
        new_menu_button(Rect::new(550, 430, 220, 30), "rules", None,
            move || {
                let quit = show_description(&surface2).unwrap();
                if quit {
                    Some(Effect::Quit)
                } else {
                    Some(Effect::Redraw(vec![rect]))
                }
            }
        )
    }));
    container.add(Box::new({
        let surface2 = surface.clone();
        let storage2 = storage.clone();
        new_menu_button(Rect::new(550, 460, 220, 30), "options", None,
            move || {
                let quit = show_options_window(&surface2, &mut storage2.borrow_mut()).unwrap();
                if quit {
                    Some(Effect::Quit)
                } else {
                    Some(Effect::Redraw(vec![rect]))
                }
            }
        )
    }));
    container.add(Box::new({
        let surface2 = surface.clone();
        new_menu_button(Rect::new(550, 490, 220, 30), "about", None,
        move || {
            show_about(&surface2).unwrap();
            Some(Effect::Redraw(vec![rect]))
        })
    }));
    container.add(Box::new(new_menu_button(Rect::new(550, 520, 220, 30), "exit", Some(Key::Escape),
        || Some(Effect::Quit)
    )));

    Ok(container)
}

pub fn menu(surface: Rc<Surface>, storage: Rc<RefCell<Storage>>) -> Result<bool> {
    let menu = make_menu(surface.clone(), storage.clone())?;
    main_loop(&surface, &menu)
}
