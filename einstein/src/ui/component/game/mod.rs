use std::time::{Duration, Instant};
use std::rc::Rc;
use std::collections::HashSet;
use serde_derive::{Serialize, Deserialize};
use rand::thread_rng;
use crate::cell::RefCell;
use sdl2::keyboard::Keycode;
use crate::rules::{Rule, SolvedPuzzle, Possibilities, apply};
use crate::puzzle_gen::generate_puzzle;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::widget::container::Container;
use crate::ui::layout::grid::GridBuilder;
use crate::ui::component::dialog::{DialogResult, cond_dialog, DialogTheme};
use crate::ui::component::puzzle::puzzle::new_puzzle_widget;
use crate::ui::component::puzzle::puzzle_cell::PuzzleAction;
use crate::ui::component::rules_grid::{create_horizontal_rules, create_vertical_rules};
use crate::ui::component::game_title::GameTitle;
use crate::ui::component::game_button::{new_game_button};
use crate::ui::component::help_dialog::new_help_dialog;
use crate::ui::component::save_dialog::{new_save_game_dialog};
use crate::ui::component::options_dialog::{new_options_dialog};
use crate::ui::component::pause_dialog::new_pause_dialog;
use crate::ui::component::failure_dialog::{new_failure_dialog, FailureChoice};
use crate::ui::component::message_dialog::{create_message_dialog};
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

    pub fn stop(&mut self) -> Option<u32> {
        let hinted = self.hinted;
        if let Some(started_at) = self.started {
            self.elapsed += Instant::now() - started_at;
            self.started = None;
            self.hinted = true;
        }
        if !hinted {
            Some(self.elapsed.as_secs() as u32)
        } else {
            None
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

fn game_popup_background<A: 'static>(messages: &'static Messages, state: &Rc<RefCell<GamePrivate>>) -> Container<A> {
    Container::screen_modal(Background::Image(&RAIN, None))
        .add(8, 10,
            GameTitle::new(messages.einstein_puzzle, state.clone()).no_action()
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
    let container = Container::container(Size::new(308, 70), None, None);
    GridBuilder::new(container, 3, 2)
        .add(0, 0, new_game_button(messages.pause, &[], MenuAction::Pause))
        .add(1, 0, new_game_button(messages.switch, &[], MenuAction::ToggleExcluded))
        .add(2, 0, new_game_button(messages.exit, &[Keycode::Escape], MenuAction::Exit))
        .add(0, 1, new_game_button(messages.save, &[], MenuAction::Save))
        .add(1, 1, new_game_button(messages.options, &[], MenuAction::Options))
        .add(2, 1, new_game_button(messages.help, &[], MenuAction::Help))
        .build()
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

    let mut container = Container::<()>::screen_modal(Background::Image(&RAIN, None));

    container.push(8, 10,
        GameTitle::new(messages.einstein_puzzle, state.clone()).no_action()
    );

    container.push(12, 68, {
        let state2 = state.clone();
        let victory_trigger2 = victory_trigger.clone();
        let failure_trigger2 = failure_trigger.clone();
        new_puzzle_widget(&state)
            .flat_map_action(move |puzzle_action, context| {
                let score = state2.borrow_mut().stop();
                match *puzzle_action {
                    PuzzleAction::Victory => {
                        context.audio().play(&*context.resource_manager().chunk(&APPLAUSE)).unwrap();
                        *victory_trigger2.borrow_mut() = Some(score);
                    },
                    PuzzleAction::Failure => {
                        context.audio().play(&*context.resource_manager().chunk(&GLASS)).unwrap();
                        *failure_trigger2.borrow_mut() = Some(());
                    }
                }
                Ok(EventReaction::update())
            })
    });

    container.push(348, 68,
        create_horizontal_rules(
            Size::new(800 - 348 - 12, 412),
            state.clone()
        ).no_action()
    );
    container.push(12, 495,
        create_vertical_rules(
            Size::new(800 - 12 * 2, 48 * 2),
            state.clone()
        ).no_action()
    );

    container.push(12, 400, {
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        make_game_menu(messages)
            .flat_map_action(move |menu_action, _| {
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
            })
    });

    container.push(0, 0, {
        let this_state1 = state.clone();
        let this_state2 = state.clone();
        cond_dialog(&pause_trigger,
            move |_| {
                game_popup_background(messages, &this_state1)
                    .add(0, 0, new_pause_dialog(messages))
            })
            .flat_map_action(move |_, _| {
                this_state2.borrow_mut().start();
                Ok(EventReaction::empty())
            })
    });

    container.push(0, 0, {
        let this_state1 = state.clone();
        let this_state2 = state.clone();
        cond_dialog(&show_help_trigger,
            move |_| {
                game_popup_background(messages, &this_state1)
                    .add(0, 0, new_help_dialog(messages))
            })
            .flat_map_action(move |_, _| {
                this_state2.borrow_mut().start();
                Ok(EventReaction::empty())
            })
    });

    container.push(0, 0, {
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state1 = state.clone();
        let this_state2 = state.clone();
        cond_dialog(&show_opts_trigger,
            move |_| {
                game_popup_background(messages, &this_state1)
                    .add(0, 0, new_options_dialog(&storage1.borrow(), messages))
            })
            .flat_map_action(move |result, context| {
                this_state2.borrow_mut().start();
                match *result {
                    DialogResult::Ok(ref options) => {
                        storage2.borrow_mut().fullscreen = options.fullscreen;
                        storage2.borrow_mut().volume = options.volume;
                        // screen->setMode(VideoMode(800, 600, 24, options.fullscreen));
                        context.audio().set_volume(options.volume);
                    },
                    DialogResult::Cancel => {},
                }
                Ok(EventReaction::empty())
            })
    });

    container.push(0, 0, {
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let state1 = state.clone();
        let this_state = state.clone();
        cond_dialog(&save_game_trigger,
            move |_|
                game_popup_background(messages, &state1)
                    .add(0, 0, new_save_game_dialog(&storage1.borrow().saved_games, messages))
            )
            .flat_map_action(move |result, _| {
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
            })
    });

    container.push(0, 0, {
        let save_score_trigger2 = save_score_trigger.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        let storage2 = storage.clone();
        cond_dialog(&victory_trigger,
            move |score| {
                let score = *score;
                create_message_dialog(DialogTheme::White, messages.won).map_action(move |_| score)
            })
            .flat_map_action(move |score, _| {
                if let Some(score) = score.filter(|score| storage2.borrow().scores.is_deserving(*score)) {
                    *save_score_trigger2.borrow_mut() = Some(score);
                } else {
                    *show_scores_trigger2.borrow_mut() = Some(None);
                }
                Ok(EventReaction::update())
            })
    });

    container.push(0, 0, {
        let show_scores_trigger2 = show_scores_trigger.clone();
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        cond_dialog(&save_score_trigger,
            move |score| {
                let score = *score;
                let last_name = match storage1.borrow().last_name {
                    Some(ref n) => n.clone(),
                    None => "anonymous".to_string()
                };
                new_player_name_dialog(&last_name, messages)
                    .map_action(move |name| (name.to_string(), score))
            })
            .flat_map_action(move |(name, score), _| {
                storage2.borrow_mut().last_name = Some(name.to_string());
                let pos = storage2.borrow_mut().scores.add_score_entry(Score { name: name.to_string(), score: *score });
                *show_scores_trigger2.borrow_mut() = Some(pos);
                Ok(EventReaction::empty())
            })
    });

    container.push(0, 0, {
        let storage2 = storage.clone();
        cond_dialog(&show_scores_trigger, move |index| create_topscores_dialog(&storage2.borrow().scores, messages, *index))
            .no_action()
    });

    container.push(0, 0, {
        let state2 = state.clone();
        cond_dialog(&failure_trigger, move |_| new_failure_dialog(messages))
            .flat_map_action(move |result, _| {
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
            })
    });

    container
}
