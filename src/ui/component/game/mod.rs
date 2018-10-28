use std::time::{Duration, Instant};
use std::rc::Rc;
use cell::RefCell;
use sdl::event::{Key};
use rules::{Rule, SolvedPuzzle, Possibilities, apply};
use puzzle_gen::generate_puzzle;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::*;
use ui::widget::conditional::*;
use ui::widget::label::Label;
use ui::widget::game_button::new_game_button;
use ui::widget::image::Image;
use ui::widget::container::Container;
use ui::component::dialog::DialogResult;
use ui::component::puzzle::puzzle::new_puzzle_widget;
use ui::component::puzzle::puzzle_cell::PuzzleAction;
use ui::component::puzzle::horizontal_rules::HorizontalRules;
use ui::component::puzzle::vertical_rules::VerticalRules;
use ui::component::watch::Watch;
use ui::component::rules_dialog::{new_help_dialog};
use ui::component::save_dialog::{new_save_game_dialog};
use ui::component::options_dialog::{new_options_dialog};
use ui::component::pause_dialog::new_pause_dialog;
use ui::component::failure_dialog::{new_failure_dialog, FailureChoice};
use ui::component::message_dialog::{create_message_dialog, MessageType};
use ui::component::player_name_dialog::new_player_name_dialog;
use ui::component::topscores_dialog::create_topscores_dialog;
use resources::messages::Messages;
use error::*;
use storage::*;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct HorizontalRule {
    pub is_excluded: bool,
    pub original_index: usize,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct VerticalRule {
    pub is_excluded: bool,
    pub original_index: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GamePrivate {
    pub solved_puzzle: SolvedPuzzle,
    pub rules: Vec<Rule>,
    pub possibilities: Possibilities,
    pub valid: bool,
    pub win: bool,

    pub horizontal_rules: Vec<HorizontalRule>,
    pub vertical_rules: Vec<VerticalRule>,
    pub show_excluded: bool,

    pub elapsed: Duration,
    #[serde(skip)]
    pub started: Option<Instant>,

    pub hinted: bool,
}

const RAIN: &[u8] = include_bytes!("./rain.bmp");
const TITLE_BG: &[u8] = include_bytes!("./title.bmp");

impl GamePrivate {
    pub fn new() -> Result<Rc<RefCell<GamePrivate>>> {
        let (solved_puzzle, rules) = generate_puzzle()?;

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
                Rule::Under(..) => vertical_rules.push(VerticalRule {
                    is_excluded: false,
                    original_index: index
                }),
                Rule::Near(..) |
                Rule::Between(..) |
                Rule::Direction(..) => horizontal_rules.push(HorizontalRule {
                    is_excluded: false,
                    original_index: index
                }),
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
        for mut rule in &mut self.horizontal_rules {
            rule.is_excluded = false;
        }
        for mut rule in &mut self.vertical_rules {
            rule.is_excluded = false;
        }
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

    pub fn toggle_horizontal_rule(&mut self, index: usize) -> Option<bool> {
        let rule = self.horizontal_rules.get_mut(index)?;
        if self.show_excluded != rule.is_excluded {
            return None;
        }
        rule.is_excluded = !rule.is_excluded;
        Some(rule.is_excluded)
    }

    pub fn toggle_vertical_rule(&mut self, index: usize) -> Option<bool> {
        let rule = self.vertical_rules.get_mut(index)?;
        if self.show_excluded != rule.is_excluded {
            return None;
        }
        rule.is_excluded = !rule.is_excluded;
        Some(rule.is_excluded)
    }
}

const APP_WIDTH: u32 =          800;
const TITLE_RIGHT: u32 =          9;
const TITLE_TOP: u32 =            8;
const TITLE_PADDING_RIGHT: u32 =  7;
const TITLE_PADDING_TOP: u32 =    7;
const WATCH_WIDTH: u32 =        100;
const WATCH_HEIGHT: u32 =        34;

fn game_popup<W, A, F>(trigger: &Rc<RefCell<Option<()>>>, create_widget: F, messages: &'static Messages) -> impl Widget<A>
    where
        F: Fn() -> Result<W> + 'static,
        W: Widget<A> + 'static,
        A: Clone + 'static,
{
    let screen_rect = Rect::new(0, 0, 800, 600);
    ConditionalWidget::new(
        trigger.clone(),
        move |_| Ok(Container::container(screen_rect, BackgroundPattern::Custom("RAIN", RAIN))
            .add(WidgetMapAction::no_action(
                Image::new(Rect::new(8, 10, 783, 47), TITLE_BG)?
            ))
            .add(WidgetMapAction::no_action(
                Label::title(Rect::new(20, 10, 500, 47), messages.einstein_puzzle)
            ))
            .add((create_widget)()?))
    )
}

pub fn new_game_widget(storage: Rc<RefCell<Storage>>, state: Rc<RefCell<GamePrivate>>, messages: &'static Messages) -> Result<Container<()>> {
    let screen_rect = Rect::new(0, 0, 800, 600);

    let save_game_trigger = Rc::new(RefCell::new(None));
    let show_opts_trigger = Rc::new(RefCell::new(None));
    let show_help_trigger = Rc::new(RefCell::new(None));
    let pause_trigger = Rc::new(RefCell::new(None));
    let victory_trigger = Rc::new(RefCell::new(None));
    let save_score_trigger = Rc::new(RefCell::new(None));
    let show_scores_trigger = Rc::new(RefCell::new(None));
    let failure_trigger = Rc::new(RefCell::new(None));

    let mut container = Container::<()>::modal(screen_rect, BackgroundPattern::Custom("RAIN_TILE", RAIN));

    container.push(WidgetMapAction::no_action(
        Image::new(Rect::new(8, 10, 783, 47), TITLE_BG)?
    ));

    container.push(WidgetMapAction::no_action(
        Label::title(Rect::new(20, 10, 500, 47), messages.einstein_puzzle)
    ));

    container.push(WidgetMapAction::no_action(
        Watch::new(
            Rect::new(
                (APP_WIDTH - TITLE_RIGHT - TITLE_PADDING_RIGHT - WATCH_WIDTH) as i32,
                (TITLE_TOP + TITLE_PADDING_TOP) as i32,
                WATCH_WIDTH,
                WATCH_HEIGHT
            ),
            state.clone()
        )
    ));

    container.push({
        let state2 = state.clone();
        let victory_trigger2 = victory_trigger.clone();
        let failure_trigger2 = failure_trigger.clone();
        WidgetMapAction::new(
            new_puzzle_widget(state.clone())?,
            move |puzzle_action| {
                state2.borrow_mut().stop();
                match *puzzle_action {
                    PuzzleAction::Victory => {
                        // sound->play(L"applause.wav");
                        *victory_trigger2.borrow_mut() = Some(());
                    },
                    PuzzleAction::Failure => {
                        // sound->play(L"glasbk2.wav");
                        *failure_trigger2.borrow_mut() = Some(());
                    }
                }
                Ok(EventReaction::empty())
            }
        )
    });

    container.push(WidgetMapAction::no_action(
        HorizontalRules::new(
            Rect::new(348, 68, 800 - 348 - 12, 412),
            state.clone()
        )?
    ));
    container.push(WidgetMapAction::no_action(
        VerticalRules::new(
            Rect::new(12, 495, 800 - 12 * 2, 48 * 2),
            state.clone()
        )?
    ));

    container.push({
        let this_state = state.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(12, 440, 94, 30), messages.save, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *save_game_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let this_state = state.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(119, 400, 94, 30), messages.switch, None, ()),
            move |_| {
                this_state.borrow_mut().toggle_show_excluded();
                Ok(EventReaction::empty())
            }
        )
    });

    container.push(
        new_game_button(Rect::new(226, 400, 94, 30), messages.exit, Some(Key::Escape), ())
    );

    container.push({
        let this_state = state.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(226, 440, 94, 30), messages.help, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *show_help_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let this_state = state.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(119, 440, 94, 30), messages.options, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *show_opts_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(12, 400, 94, 30), messages.pause, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *pause_trigger2.borrow_mut() = Some(());
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        WidgetMapAction::new(
            game_popup(&pause_trigger, move || new_pause_dialog(messages), messages),
            move |_| {
                *pause_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let this_state = state.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            game_popup(&show_help_trigger, move || new_help_dialog(messages), messages),
            move |_| {
                *show_help_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state = state.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            game_popup(&show_opts_trigger, move || new_options_dialog(&storage1.borrow(), messages), messages),
            move |result| {
                *show_opts_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
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
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state = state.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        WidgetMapAction::new(
            game_popup(
                &save_game_trigger,
                move || new_save_game_dialog(&storage1.borrow().saved_games, messages),
                messages
            ),
            move |result| {
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

    container.push({
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
            move |_| {
                *victory_trigger2.borrow_mut() = None;
                let score = state2.borrow().elapsed.as_secs() as u32;
                if !state2.borrow().hinted && storage2.borrow().scores.is_deserving(score) {
                    *save_score_trigger2.borrow_mut() = Some(score);
                } else {
                    *show_scores_trigger2.borrow_mut() = Some(None);
                }
                Ok(EventReaction::empty())
            }
        )
    });

    container.push({
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
            move |name| {
                let score = save_score_trigger2.borrow().unwrap_or(0);
                *save_score_trigger2.borrow_mut() = None;
                storage2.borrow_mut().last_name = Some(name.to_string());
                let pos = storage2.borrow_mut().scores.add_score_entry(Score { name: name.to_string(), score });
                *show_scores_trigger2.borrow_mut() = Some(pos);
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
                move |index| create_topscores_dialog(&storage2.borrow().scores, messages, *index)
            ),
            move |_| {
                *show_scores_trigger2.borrow_mut() = None;
                Ok(EventReaction::action(()))
            }
        )
    });

    container.push({
        let failure_trigger2 = failure_trigger.clone();
        let state2 = state.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                failure_trigger.clone(),
                move |_| new_failure_dialog(messages)
            ),
            move |result| {
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

    Ok(container)
}

/*
void Game::pleaseWait()
{
    drawWallpaper(screen, L"rain.bmp");
    Window window(screen, 230, 260, 340, 80, L"greenpattern.bmp", 6);
    window.draw();
    Font font(L"laudcn2.ttf", 16);
    Label label(screen, &font, 280, 275, 240, 50, Label::ALIGN_CENTER,
                Label::ALIGN_MIDDLE, 255,255,0, msg(L"loading"));
    label.draw();
    screen->addRegionToUpdate(0, 0, screen->getWidth(), screen->getHeight());
    screen->flush();
}
*/
