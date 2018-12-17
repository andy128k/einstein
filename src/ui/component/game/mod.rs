use std::time::{Duration, Instant};
use std::rc::Rc;
use std::collections::HashSet;
use serde_derive::{Serialize, Deserialize};
use rand::thread_rng;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use crate::rules::{Rule, SolvedPuzzle, Possibilities, apply};
use crate::puzzle_gen::generate_puzzle;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::widget::conditional::*;
use crate::ui::widget::grid::new_grid;
use crate::ui::widget::game_button::{new_game_button, GAME_BUTTON_SIZE};
use crate::ui::widget::container::Container;
use crate::ui::component::dialog::DialogResult;
use crate::ui::component::puzzle::puzzle::new_puzzle_widget;
use crate::ui::component::puzzle::puzzle_cell::PuzzleAction;
use crate::ui::component::rules_grid::{create_horizontal_rules, create_vertical_rules};
use crate::ui::component::game_title::GameTitle;
use crate::ui::component::help_dialog::new_help_dialog;
use crate::ui::component::save_dialog::{new_save_game_dialog};
use crate::ui::component::options_dialog::{new_options_dialog};
use crate::ui::component::pause_dialog::new_pause_dialog;
use crate::ui::component::failure_dialog::{new_failure_dialog, FailureChoice};
use crate::ui::component::message_dialog::{create_message_dialog, MessageType};
use crate::ui::component::player_name_dialog::new_player_name_dialog;
use crate::ui::component::topscores_dialog::create_topscores_dialog;
use crate::resources::manager::Resource;
use crate::resources::messages::Messages;
use crate::resources::audio::{APPLAUSE, GLASS};
use crate::error::*;
use crate::storage::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GamePrivate {
    pub solved_puzzle: SolvedPuzzle,
    pub rules: Vec<Rule>,
    pub possibilities: Possibilities,
    pub valid: bool,
    pub win: bool,

    pub horizontal_rules: Vec<usize>,
    pub vertical_rules: Vec<usize>,
    pub excluded: HashSet<usize>,
    pub show_excluded: bool,

    pub elapsed: Duration,
    #[serde(skip)]
    pub started: Option<Instant>,

    pub hinted: bool,
}

const RAIN: Resource = resource!("./rain.bmp");

impl GamePrivate {
    pub fn new() -> Result<Rc<RefCell<GamePrivate>>> {
        let mut rng = thread_rng();
        let (solved_puzzle, rules) = generate_puzzle(&mut rng)?;

        let mut possibilities = Possibilities::new();
        for rule in &rules {
            if let Rule::Open(..) = *rule {
                possibilities = apply(&possibilities, rule);
            }
        }

        let mut vertical_rules = Vec::new();
        let mut horizontal_rules = Vec::new();
        for (index, rule) in rules.iter().enumerate() {
            match *rule {
                Rule::Under(..) => vertical_rules.push(index),
                Rule::Near(..) |
                Rule::Between(..) |
                Rule::Direction(..) => horizontal_rules.push(index),
                _ => {}
            }
        }

        Ok(Rc::new(RefCell::new(GamePrivate {
            solved_puzzle,
            rules,
            possibilities,
            valid: true,
            win: false,
            elapsed: Duration::new(0, 0),
            started: None,
            vertical_rules,
            horizontal_rules,
            excluded: HashSet::new(),
            show_excluded: false,
            hinted: false,
        })))
    }

    pub fn restart(&mut self) {
        let mut possibilities = Possibilities::new();
        for rule in &self.rules {
            if let Rule::Open(..) = *rule {
                possibilities = apply(&possibilities, rule);
            }
        }
        self.possibilities = possibilities;
        self.excluded.clear();
        self.show_excluded = false;
        self.hinted = true;
        self.reset();
    }

    pub fn is_valid(&self) -> bool {
        self.possibilities.is_valid(&self.solved_puzzle)
    }

    pub fn start(&mut self) {
        if self.started.is_none() {
            self.started = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(started_at) = self.started {
            self.elapsed += Instant::now() - started_at;
            self.started = None;
            self.hinted = true;
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::new(0, 0);
        self.started = Some(Instant::now());
    }

    pub fn get_current_duration(&self) -> Duration {
        match self.started {
            Some(started_at) => self.elapsed + (Instant::now() - started_at),
            None => self.elapsed
        }
    }

    pub fn toggle_show_excluded(&mut self) {
        self.show_excluded = !self.show_excluded;
    }

    pub fn toggle_rule(&mut self, index: usize) -> Option<bool> {
        let excluded = self.excluded.contains(&index);
        if self.show_excluded != excluded {
            return None;
        }
        if excluded {
            self.excluded.remove(&index);
            Some(false)
        } else {
            self.excluded.insert(index);
            Some(true)
        }
    }
}

fn game_popup<W, A, F>(trigger: &Rc<RefCell<Option<()>>>, create_widget: F, messages: &'static Messages, state: Rc<RefCell<GamePrivate>>) -> impl Widget<A>
    where
        F: Fn() -> W + 'static,
        W: Widget<A> + 'static,
        A: Clone + 'static,
{
    ConditionalWidget::new(
        trigger.clone(),
        move |_| Container::screen_modal(Background::Pattern(&RAIN, false))
            .add(8, 10, WidgetMapAction::no_action(
                GameTitle::new(messages.einstein_puzzle, state.clone())
            ))
            .add(0, 0, (create_widget)())
    )
}

#[derive(Clone)]
enum MenuAction {
    Pause,
    ToggleExcluded,
    Exit,
    Save,
    Options,
    Help,
}

fn make_game_menu(messages: &'static Messages) -> Container<MenuAction> {
    new_grid(Size::new(308, 70), GAME_BUTTON_SIZE, 3, 2, vec![
        new_game_button(messages.pause, None, MenuAction::Pause),
        new_game_button(messages.switch, None, MenuAction::ToggleExcluded),
        new_game_button(messages.exit, Some(Keycode::Escape), MenuAction::Exit),
        new_game_button(messages.save, None, MenuAction::Save),
        new_game_button(messages.options, None, MenuAction::Options),
        new_game_button(messages.help, None, MenuAction::Help),
    ])
}

pub fn new_game_widget(storage: Rc<RefCell<Storage>>, state: Rc<RefCell<GamePrivate>>, messages: &'static Messages) -> Container<()> {
    let save_game_trigger = Rc::new(RefCell::new(None));
    let show_opts_trigger = Rc::new(RefCell::new(None));
    let show_help_trigger = Rc::new(RefCell::new(None));
    let pause_trigger = Rc::new(RefCell::new(None));
    let victory_trigger = Rc::new(RefCell::new(None));
    let save_score_trigger = Rc::new(RefCell::new(None));
    let show_scores_trigger = Rc::new(RefCell::new(None));
    let failure_trigger = Rc::new(RefCell::new(None));

    let mut container = Container::<()>::screen_modal(Background::Pattern(&RAIN, false));

    container.push(8, 10, WidgetMapAction::no_action(
        GameTitle::new(messages.einstein_puzzle, state.clone())
    ));

    container.push(12, 68, {
        let state2 = state.clone();
        let victory_trigger2 = victory_trigger.clone();
        let failure_trigger2 = failure_trigger.clone();
        WidgetMapAction::new(
            new_puzzle_widget(&state),
            move |puzzle_action, resource_manager, audio| {
                state2.borrow_mut().stop();
                match *puzzle_action {
                    PuzzleAction::Victory => {
                        audio.play(&*resource_manager.chunk(&APPLAUSE)).unwrap();
                        *victory_trigger2.borrow_mut() = Some(());
                    },
                    PuzzleAction::Failure => {
                        audio.play(&*resource_manager.chunk(&GLASS)).unwrap();
                        *failure_trigger2.borrow_mut() = Some(());
                    }
                }
                Ok(EventReaction::update())
            }
        )
    });

    container.push(348, 68, WidgetMapAction::no_action(
        create_horizontal_rules(
            Size::new(800 - 348 - 12, 412),
            state.clone()
        )
    ));
    container.push(12, 495, WidgetMapAction::no_action(
        create_vertical_rules(
            Size::new(800 - 12 * 2, 48 * 2),
            state.clone()
        )
    ));

    container.push(12, 400, {
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            make_game_menu(messages),
            move |menu_action, _, _| {
                match menu_action {
                    MenuAction::Pause => {
                        this_state.borrow_mut().stop();
                        *pause_trigger2.borrow_mut() = Some(());
                        Ok(EventReaction::empty())
                    },
                    MenuAction::ToggleExcluded => {
                        this_state.borrow_mut().toggle_show_excluded();
                        Ok(EventReaction::empty())
                    },
                    MenuAction::Exit => {
                        Ok(EventReaction::action(()))
                    },
                    MenuAction::Save => {
                        this_state.borrow_mut().stop();
                        *save_game_trigger2.borrow_mut() = Some(());
                        Ok(EventReaction::empty())
                    },
                    MenuAction::Options => {
                        this_state.borrow_mut().stop();
                        *show_opts_trigger2.borrow_mut() = Some(());
                        Ok(EventReaction::empty())
                    },
                    MenuAction::Help => {
                        this_state.borrow_mut().stop();
                        *show_help_trigger2.borrow_mut() = Some(());
                        Ok(EventReaction::empty())
                    },
                }
            }
        )
    });

    container.push(0, 0, {
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        WidgetMapAction::new(
            game_popup(&pause_trigger, move || new_pause_dialog(messages), messages, state.clone()),
            move |_, _, _| {
                *pause_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                Ok(EventReaction::empty())
            }
        )
    });

    container.push(0, 0, {
        let this_state = state.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            game_popup(&show_help_trigger, move || new_help_dialog(messages), messages, state.clone()),
            move |_, _, _| {
                *show_help_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                Ok(EventReaction::empty())
            }
        )
    });

    container.push(0, 0, {
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state = state.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            game_popup(&show_opts_trigger, move || new_options_dialog(&storage1.borrow(), messages), messages, state.clone()),
            move |result, _, audio| {
                *show_opts_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
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
            }
        )
    });

    container.push(0, 0, {
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state = state.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        WidgetMapAction::new(
            game_popup(
                &save_game_trigger,
                move || new_save_game_dialog(&storage1.borrow().saved_games, messages),
                messages,
                state.clone()
            ),
            move |result, _, _| {
                *save_game_trigger2.borrow_mut() = None;
                match *result {
                    DialogResult::Ok((index, ref name)) => {
                        storage2.borrow_mut().saved_games[index] = Some(SavedGame {
                            name: name.to_owned(),
                            game: this_state.borrow().clone()
                        });
                    },
                    DialogResult::Cancel => {}
                }
                this_state.borrow_mut().start();
                Ok(EventReaction::empty())
            }
        )
    });

    container.push(0, 0, {
        let victory_trigger2 = victory_trigger.clone();
        let save_score_trigger2 = save_score_trigger.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        let state2 = state.clone();
        let storage2 = storage.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                victory_trigger.clone(),
                move |_| create_message_dialog(MessageType::Neutral, messages.won)
            ),
            move |_, _, _| {
                *victory_trigger2.borrow_mut() = None;
                let score = state2.borrow().elapsed.as_secs() as u32;
                if !state2.borrow().hinted && storage2.borrow().scores.is_deserving(score) {
                    *save_score_trigger2.borrow_mut() = Some(score);
                } else {
                    *show_scores_trigger2.borrow_mut() = Some(None);
                }
                Ok(EventReaction::update())
            }
        )
    });

    container.push(0, 0, {
        let save_score_trigger2 = save_score_trigger.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                save_score_trigger.clone(),
                move |_| {
                    let last_name = match storage1.borrow().last_name {
                        Some(ref n) => n.clone(),
                        None => "anonymous".to_string()
                    };
                    new_player_name_dialog(&last_name, messages)
                }
            ),
            move |name, _, _| {
                let score = save_score_trigger2.borrow().unwrap_or(0);
                *save_score_trigger2.borrow_mut() = None;
                storage2.borrow_mut().last_name = Some(name.to_string());
                let pos = storage2.borrow_mut().scores.add_score_entry(Score { name: name.to_string(), score });
                *show_scores_trigger2.borrow_mut() = Some(pos);
                Ok(EventReaction::empty())
            }
        )
    });

    container.push(0, 0, {
        let storage2 = storage.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_scores_trigger.clone(),
                move |index| create_topscores_dialog(&storage2.borrow().scores, messages, *index)
            ),
            move |_, _, _| {
                *show_scores_trigger2.borrow_mut() = None;
                Ok(EventReaction::action(()))
            }
        )
    });

    container.push(0, 0, {
        let failure_trigger2 = failure_trigger.clone();
        let state2 = state.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                failure_trigger.clone(),
                move |_| new_failure_dialog(messages)
            ),
            move |result, _, _| {
                *failure_trigger2.borrow_mut() = None;
                match *result {
                    FailureChoice::StartNew => {
                        let g = GamePrivate::new().unwrap();
                        *state2.borrow_mut() = g.borrow().clone();
                        Ok(EventReaction::empty())
                    },
                    FailureChoice::TryAgain => {
                        state2.borrow_mut().restart();
                        Ok(EventReaction::empty())
                    },
                    FailureChoice::Cancel => Ok(EventReaction::action(())),
                }
            }
        )
    });

    container
}
