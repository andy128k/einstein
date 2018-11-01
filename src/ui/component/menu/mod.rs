use std::rc::Rc;
use cell::RefCell;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use error::*;
use storage::*;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::widget::*;
use ui::widget::common::*;
use ui::widget::menu_button::*;
use ui::widget::conditional::*;
use ui::widget::container::Container;
use ui::widget::label::*;
use ui::main_loop::MainLoopQuit;
use ui::component::dialog::*;
use ui::component::game::{new_game_widget, GamePrivate};
use ui::component::load_dialog::{new_load_game_dialog};
use ui::component::topscores_dialog::{create_topscores_dialog};
use ui::component::rules_dialog::{new_help_dialog};
use ui::component::options_dialog::{new_options_dialog};
use ui::component::about_dialog::{create_about_dialog};
use resources::manager::Resource;
use resources::messages::Messages;

const MENU_BG: Resource = resource!("./nova.bmp");

pub fn make_menu(messages: &'static Messages, storage: Rc<RefCell<Storage>>) -> Result<Container<MainLoopQuit>> {
    let rect = Rect::new(0, 0, 800, 600);

    let new_game_trigger = Rc::new(RefCell::new(None));
    let load_game_trigger = Rc::new(RefCell::new(None));
    let show_scores_trigger = Rc::new(RefCell::new(None));
    let show_help_trigger = Rc::new(RefCell::new(None));
    let show_opts_trigger = Rc::new(RefCell::new(None));
    let show_about_trigger = Rc::new(RefCell::new(None));

    let mut container = Container::<MainLoopQuit>::modal(rect, Background::Pattern(&MENU_BG, false));

    container.push(WidgetMapAction::no_action(
        Label::title(Rect::new(0, 30, 800, 30), messages.einstein_flowix)
    ));

    container.push(WidgetMapAction::no_action(
        Label::new(Rect::new(0, 60, 800, 30), "http://games.flowix.com", Color::RGB(255, 255, 0), HorizontalAlign::Center)
    ));

    container.push({
        let new_game_trigger2 = new_game_trigger.clone();
        WidgetMapAction::new(
            new_menu_button(Rect::new(550, 340, 220, 30), messages.new_game, None, ()),
            move |_| {
                let game = GamePrivate::new().unwrap();
                *new_game_trigger2.borrow_mut() = Some(game);
                Ok(EventReaction::empty())
            }
        )
    });
    container.push({
        let load_game_trigger2 = load_game_trigger.clone();
        WidgetMapAction::new(
            new_menu_button(Rect::new(550, 370, 220, 30), messages.load_game, None, ()),
            move |_| {
                *load_game_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });
    container.push({
        let show_scores_trigger2 = show_scores_trigger.clone();
        WidgetMapAction::new(
            new_menu_button(Rect::new(550, 400, 220, 30), messages.top_scores, None, ()),
            move |_| {
                *show_scores_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });
    container.push({
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            new_menu_button(Rect::new(550, 430, 220, 30), messages.rules, None, ()),
            move |_| {
                *show_help_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });
    container.push({
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            new_menu_button(Rect::new(550, 460, 220, 30), messages.options, None, ()),
            move |_| {
                *show_opts_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });
    container.push({
        let show_about_trigger2 = show_about_trigger.clone();
        WidgetMapAction::new(
            new_menu_button(Rect::new(550, 490, 220, 30), messages.about, None, ()),
            move |_| {
                *show_about_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });
    container.push(
        new_menu_button(Rect::new(550, 520, 220, 30), messages.exit, Some(Keycode::Escape), MainLoopQuit)
    );

    container.push({
        let storage2 = storage.clone();
        let new_game_trigger2 = new_game_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                new_game_trigger.clone(),
                move |game| {
                    game.borrow_mut().start();
                    let game_widget = new_game_widget(storage2.clone(), game.clone(), messages)?;
                    Ok(game_widget)
                }
            ),
            move |_| {
                *new_game_trigger2.borrow_mut() = None;
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let storage2 = storage.clone();
        let load_game_trigger2 = load_game_trigger.clone();
        let new_game_trigger2 = new_game_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                load_game_trigger.clone(),
                move |_| {
                    let load_dialog = new_load_game_dialog(&storage2.borrow().saved_games, messages)?;
                    Ok(load_dialog)
                }
            ),
            move |result| {
                *load_game_trigger2.borrow_mut() = None;
                match *result {
                    DialogResult::Ok(ref game_data) => {
                        let game = Rc::new(RefCell::new(game_data.clone()));
                        game.borrow_mut().hinted = true;
                        *new_game_trigger2.borrow_mut() = Some(game);
                    },
                    DialogResult::Cancel => {},
                }
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let storage2 = storage.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_scores_trigger.clone(),
                move |_| create_topscores_dialog(&storage2.borrow().scores, messages, None)
            ),
            move |_| {
                *show_scores_trigger2.borrow_mut() = None;
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_help_trigger.clone(),
                move |_| new_help_dialog(messages)
            ),
            move |_| {
                *show_help_trigger2.borrow_mut() = None;
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_opts_trigger.clone(),
                move |_| new_options_dialog(&storage1.borrow(), messages)
            ),
            move |result| {
                *show_opts_trigger2.borrow_mut() = None;
                match *result {
                    DialogResult::Ok(ref options) => {
                        storage2.borrow_mut().fullscreen = options.fullscreen;
                        storage2.borrow_mut().volume = options.volume;
                        // screen->setMode(VideoMode(800, 600, 24, options.fullscreen));
                        // sound->setVolume(options.volume);
                    },
                    DialogResult::Cancel => {},
                }
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let show_about_trigger2 = show_about_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_about_trigger.clone(),
                move |_| create_about_dialog(messages)
            ),
            move |_| {
                *show_about_trigger2.borrow_mut() = None;
                Ok(EventReaction::empty())
            }
        )
    });

    Ok(container)
}
