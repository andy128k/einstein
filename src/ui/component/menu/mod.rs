use std::rc::Rc;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::error::*;
use crate::storage::*;
use crate::ui::context::{Size, HorizontalAlign};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::widget::menu_button::*;
use crate::ui::widget::grid::new_grid;
use crate::ui::widget::container::Container;
use crate::ui::widget::label::*;
use crate::ui::main_loop::MainLoopQuit;
use crate::ui::component::dialog::*;
use crate::ui::component::game::{new_game_widget, GamePrivate};
use crate::ui::component::load_dialog::{new_load_game_dialog};
use crate::ui::component::topscores_dialog::{create_topscores_dialog};
use crate::ui::component::help_dialog::new_help_dialog;
use crate::ui::component::options_dialog::{new_options_dialog};
use crate::ui::component::about_dialog::{create_about_dialog};
use crate::resources::manager::Resource;
use crate::resources::messages::Messages;

const MENU_BG: Resource = resource!("./nova.bmp");

#[derive(Clone)]
enum MainMenuAction {
    NewGame,
    LoadGame,
    ShowScores,
    Help,
    Options,
    About,
    Exit,
}

pub fn make_menu(messages: &'static Messages, storage: Rc<RefCell<Storage>>) -> Result<Container<MainLoopQuit>> {
    let new_game_trigger = Rc::new(RefCell::new(None));
    let load_game_trigger = Rc::new(RefCell::new(None));
    let show_scores_trigger = Rc::new(RefCell::new(None));
    let show_help_trigger = Rc::new(RefCell::new(None));
    let show_opts_trigger = Rc::new(RefCell::new(None));
    let show_about_trigger = Rc::new(RefCell::new(None));

    let mut container = Container::<MainLoopQuit>::modal(Size::new(800, 600), Background::Pattern(&MENU_BG, false));

    container.push(0, 30, WidgetMapAction::no_action(
        Label::title(Size::new(800, 30), messages.einstein_flowix)
    ));

    container.push(0, 60, WidgetMapAction::no_action(
        Label::new(Size::new(800, 30), "http://games.flowix.com", Color::RGB(255, 255, 0), HorizontalAlign::Center)
    ));

    container.push(550, 340, {
        let new_game_trigger2 = new_game_trigger.clone();
        let load_game_trigger2 = load_game_trigger.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        let show_about_trigger2 = show_about_trigger.clone();
        new_grid(Size::new(220, 210), Size::new(220, 30), 1, 7, vec![
            new_menu_button(Size::new(220, 30), messages.new_game, None, MainMenuAction::NewGame),
            new_menu_button(Size::new(220, 30), messages.load_game, None, MainMenuAction::LoadGame),
            new_menu_button(Size::new(220, 30), messages.top_scores, None, MainMenuAction::ShowScores),
            new_menu_button(Size::new(220, 30), messages.rules, None, MainMenuAction::Help),
            new_menu_button(Size::new(220, 30), messages.options, None, MainMenuAction::Options),
            new_menu_button(Size::new(220, 30), messages.about, None, MainMenuAction::About),
            new_menu_button(Size::new(220, 30), messages.exit, Some(Keycode::Escape), MainMenuAction::Exit),
        ]).flat_map_action(move |menu_action, _, _| {
            match menu_action {
                MainMenuAction::NewGame => {
                    let game = GamePrivate::new().unwrap();
                    *new_game_trigger2.borrow_mut() = Some(game);
                    Ok(EventReaction::empty())
                },
                MainMenuAction::LoadGame => {
                    *load_game_trigger2.borrow_mut() = Some(());
                    Ok(EventReaction::empty())
                },
                MainMenuAction::ShowScores => {
                    *show_scores_trigger2.borrow_mut() = Some(());
                    Ok(EventReaction::empty())
                },
                MainMenuAction::Help => {
                    *show_help_trigger2.borrow_mut() = Some(());
                    Ok(EventReaction::empty())
                },
                MainMenuAction::Options => {
                    *show_opts_trigger2.borrow_mut() = Some(());
                    Ok(EventReaction::empty())
                },
                MainMenuAction::About => {
                    *show_about_trigger2.borrow_mut() = Some(());
                    Ok(EventReaction::empty())
                },
                MainMenuAction::Exit => {
                    Ok(EventReaction::action(MainLoopQuit))
                },
            }
        })
    });

    container.push(0, 0, {
        let storage2 = storage.clone();
        cond_dialog(&new_game_trigger, move |game| {
            game.borrow_mut().start();
            new_game_widget(storage2.clone(), game.clone(), messages)
        }).no_action()
    });

    container.push(0, 0, {
        let storage2 = storage.clone();
        let new_game_trigger2 = new_game_trigger.clone();
        cond_dialog(&load_game_trigger, move |_| new_load_game_dialog(&storage2.borrow().saved_games, messages))
            .flat_map_action(move |result, _, _| {
                match *result {
                    DialogResult::Ok(ref game_data) => {
                        let game = Rc::new(RefCell::new(game_data.clone()));
                        game.borrow_mut().hinted = true;
                        *new_game_trigger2.borrow_mut() = Some(game);
                    },
                    DialogResult::Cancel => {},
                }
                Ok(EventReaction::empty())
            })
    });

    container.push(0, 0, {
        let storage2 = storage.clone();
        cond_dialog(&show_scores_trigger, move |_| create_topscores_dialog(&storage2.borrow().scores, messages, None)).no_action()
    });

    container.push(0, 0,
        cond_dialog(&show_help_trigger, move |_| new_help_dialog(messages)).no_action()
    );

    container.push(0, 0, {
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        cond_dialog(&show_opts_trigger, move |_| new_options_dialog(&storage1.borrow(), messages))
            .flat_map_action(move |result, _, audio| {
                match *result {
                    DialogResult::Ok(ref options) => {
                        storage2.borrow_mut().fullscreen = options.fullscreen;
                        storage2.borrow_mut().volume = options.volume;
                        // screen->setMode(VideoMode(800, 600, 24, options.fullscreen));
                        audio.set_volume(options.volume);
                    },
                    DialogResult::Cancel => {},
                }
                Ok(EventReaction::empty())
            })
    });

    container.push(0, 0,
        cond_dialog(&show_about_trigger, move |_| create_about_dialog(messages)).no_action()
    );

    Ok(container)
}
