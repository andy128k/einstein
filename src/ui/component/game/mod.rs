use std::time::{Duration, Instant};
use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use rules::{Rule, SolvedPuzzle, Possibilities, Thing, apply};
use puzzle_gen::generate_puzzle;
use ui::widget::widget::*;
use ui::widget::dialog::*;
use ui::widget::window::Window;
use ui::widget::title::Title;
use ui::widget::button::*;
use ui::widget::game_button::new_game_button;
use ui::widget::image::Image;
use ui::widget::label::Label;
use ui::utils::{load_image, draw_text, HorizontalAlign, VerticalAlign, adjust_brightness};
use ui::main_loop::{main_loop, ModalResult};
use ui::component::dialog::DialogResult;
use ui::component::puzzle::puzzle::new_puzzle_widget;
use ui::component::puzzle::puzzle_cell::PuzzleAction;
use ui::component::puzzle::horizontal_rules::HorizontalRules;
use ui::component::puzzle::vertical_rules::VerticalRules;
use ui::component::watch::Watch;
use ui::component::rules_dialog::{show_description, new_help_dialog};
use ui::component::save_dialog::{save_game, new_save_game_dialog};
use ui::component::options_dialog::{new_options_dialog, show_options_window};
use ui::component::pause_dialog::new_pause_dialog;
use ui::component::failure_dialog::{new_failure_dialog, show_failure_dialog, FailureChoice};
use ui::component::message_dialog::{create_message_dialog, MessageType};
use ui::component::player_name_dialog::new_player_name_dialog;
use ui::component::topscores_dialog::create_topscores_dialog;
use resources::messages::{get_messages, Messages};
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

const RAIN_TILE: &[u8] = include_bytes!("./rain.bmp");
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

pub fn new_game_widget(storage: Rc<RefCell<Storage>>, state: Rc<RefCell<GamePrivate>>, messages: &'static Messages) -> Result<WidgetPtr<ModalResult<()>>> {
    let screen_rect = Rect::new(0, 0, 800, 600);

    let save_game_trigger = Rc::new(RefCell::new(None));
    let show_opts_trigger = Rc::new(RefCell::new(None));
    let show_help_trigger = Rc::new(RefCell::new(None));
    let pause_trigger = Rc::new(RefCell::new(None));
    let victory_trigger = Rc::new(RefCell::new(None));
    let save_score_trigger = Rc::new(RefCell::new(None));
    let show_scores_trigger = Rc::new(RefCell::new(None));
    let failure_trigger = Rc::new(RefCell::new(None));

    let mut container: Vec<WidgetPtr<ModalResult<()>>> = Vec::new();

    container.push(Box::new(
        InterceptWidget::default()
    ));

    container.push(Box::new(WidgetMapAction::no_action(
        Image::new(screen_rect, RAIN_TILE)?
    )));

    container.push(Box::new(WidgetMapAction::no_action(
        Image::new(Rect::new(8, 10, 783, 47), TITLE_BG)?
    )));

    container.push(Box::new(WidgetMapAction::no_action(
        Title {
            text: messages.einstein_puzzle.to_string(),
            rect: Rect::new(20, 10, 500, 47),
        }
    )));

    container.push(Box::new(WidgetMapAction::no_action(
        Watch::new(state.clone())
    )));

    container.push(Box::new({
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
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new(WidgetMapAction::no_action(
        HorizontalRules::new(state.clone())?
    )));
    container.push(Box::new(WidgetMapAction::no_action(
        VerticalRules::new(state.clone())?
    )));

    container.push(Box::new({
        let this_state = state.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(12, 440, 94, 30), messages.save, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *save_game_trigger2.borrow_mut() = Some(());
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let this_state = state.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(119, 400, 94, 30), messages.switch, None, ()),
            move |_| {
                this_state.borrow_mut().toggle_show_excluded();
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new(
        new_game_button(Rect::new(226, 400, 94, 30), messages.exit, Some(Key::Escape), ModalResult(()))
    ));

    container.push(Box::new({
        let this_state = state.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(226, 440, 94, 30), messages.help, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *show_help_trigger2.borrow_mut() = Some(());
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let this_state = state.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(119, 440, 94, 30), messages.options, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *show_opts_trigger2.borrow_mut() = Some(());
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        WidgetMapAction::new(
            new_game_button(Rect::new(12, 400, 94, 30), messages.pause, None, ()),
            move |_| {
                this_state.borrow_mut().stop();
                *pause_trigger2.borrow_mut() = Some(());
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let this_state = state.clone();
        let pause_trigger2 = pause_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                pause_trigger.clone(),
                move |_| new_pause_dialog(messages)
            ),
            move |_| {
                *pause_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let this_state = state.clone();
        let show_help_trigger2 = show_help_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_help_trigger.clone(),
                move |_| new_help_dialog(messages)
            ),
            move |_| {
                *show_help_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state = state.clone();
        let show_opts_trigger2 = show_opts_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_opts_trigger.clone(),
                move |_| new_options_dialog(&storage1.borrow(), messages)
            ),
            move |result| {
                *show_opts_trigger2.borrow_mut() = None;
                this_state.borrow_mut().start();
                match *result {
                    ModalResult(DialogResult::Ok(ref options)) => {
                        storage2.borrow_mut().fullscreen = options.fullscreen;
                        storage2.borrow_mut().volume = options.volume;
                        // screen->setMode(VideoMode(800, 600, 24, options.fullscreen));
                        // sound->setVolume(options.volume);
                    },
                    ModalResult(DialogResult::Cancel) => {},
                }
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        let this_state = state.clone();
        let save_game_trigger2 = save_game_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                save_game_trigger.clone(),
                move |_| new_save_game_dialog(&storage1.borrow().saved_games, messages)
            ),
            move |result| {
                *save_game_trigger2.borrow_mut() = None;
                match *result {
                    ModalResult(DialogResult::Ok((index, ref name))) => {
                        storage2.borrow_mut().saved_games[index] = Some(SavedGame {
                            name: name.to_owned(),
                            game: this_state.borrow().clone()
                        });
                    },
                    ModalResult(DialogResult::Cancel) => {}
                }
                this_state.borrow_mut().start();
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
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
            move |result| {
                *victory_trigger2.borrow_mut() = None;
                let score = state2.borrow().elapsed.as_secs() as u32;
                if !state2.borrow().hinted && storage2.borrow().scores.is_deserving(score) {
                    *save_score_trigger2.borrow_mut() = Some(score);
                } else {
                    *show_scores_trigger2.borrow_mut() = Some(None);
                }
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let save_score_trigger2 = save_score_trigger.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        let state2 = state.clone();
        let storage1 = storage.clone();
        let storage2 = storage.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                save_score_trigger.clone(),
                move |score| {
                    let last_name = match storage1.borrow().last_name {
                        Some(ref n) => n.clone(),
                        None => "anonymous".to_string()
                    };
                    new_player_name_dialog(&last_name, messages)
                }
            ),
            move |result| {
                let score = save_score_trigger2.borrow().unwrap_or(0);
                *save_score_trigger2.borrow_mut() = None;
                match *result {
                    ModalResult(ref name) => {
                        storage2.borrow_mut().last_name = Some(name.to_string());
                        let pos = storage2.borrow_mut().scores.add_score_entry(Score { name: name.to_string(), score });
                        *show_scores_trigger2.borrow_mut() = Some(pos);
                    },
                };
                EventReaction::Redraw
            }
        )
    }));

    container.push(Box::new({
        let storage2 = storage.clone();
        let show_scores_trigger2 = show_scores_trigger.clone();
        WidgetMapAction::new(
            ConditionalWidget::new(
                show_scores_trigger.clone(),
                move |index| create_topscores_dialog(&storage2.borrow().scores, messages, *index)
            ),
            move |_| {
                *show_scores_trigger2.borrow_mut() = None;
                EventReaction::Action(ModalResult(()))
            }
        )
    }));

    container.push(Box::new({
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
                    ModalResult(FailureChoice::StartNew) => {
                        let g = GamePrivate::new().unwrap();
                        *state2.borrow_mut() = g.borrow().clone();
                        EventReaction::Redraw
                    },
                    ModalResult(FailureChoice::TryAgain) => {
                        state2.borrow_mut().restart();
                        EventReaction::Redraw
                    },
                    ModalResult(FailureChoice::Cancel) => EventReaction::Action(ModalResult(())),
                }
            }
        )
    }));

    Ok(Box::new(container))
}


/*
Game::Game(Screen *screen)
{
    genPuzzle();

    possibilities = ein_possibilities_new();
    possibilities = ein_possibilities_open_initials(possibilities, &rules[0], rules.size());
    
    puzzle = new Puzzle(screen, iconSet, solvedPuzzle, possibilities);
    verHints = new VertHints(screen, iconSet, rules);
    horHints = new HorHints(screen, iconSet, rules);
    watch = new Watch(screen);
}

Game::Game(Screen *screen, std::istream &stream)
    : screen(screen)
{
    pleaseWait();

    loadPuzzle(solvedPuzzle, stream);
    loadRules(rules, stream);

    ein_solved_puzzle_free(solvedPuzzle);
    solvedPuzzle = ein_solved_puzzle_clone(savedSolvedPuzzle);

    savedRules = rules;
    possibilities = ein_possibilities_new(/*stream*/);
    puzzle = new Puzzle(screen, iconSet, solvedPuzzle, possibilities);
    verHints = new VertHints(screen, iconSet, rules, stream);
    horHints = new HorHints(screen, iconSet, rules, stream);
    watch = new Watch(screen, stream);
    hinted = true;
}

void Game::save(std::ostream &stream)
{
    savePuzzle(solvedPuzzle, stream);
    saveRules(rules, stream);
    // possibilities->save(stream);
    verHints->save(stream);
    horHints->save(stream);
    watch->save(stream);
}

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

void Game::genPuzzle()
{
    pleaseWait();
    
    rules.resize(500);
    size_t rules_arr_size;
    ein_generate_puzzle(&solvedPuzzle, &rules[0], &rules_arr_size);
    rules.resize(rules_arr_size);

    savedSolvedPuzzle = ein_solved_puzzle_clone(solvedPuzzle);
    savedRules = rules;

    hinted = false;
}

void Game::resetVisuals()
{
    ein_possibilities_free(possibilities);

    possibilities = ein_possibilities_new();
    possibilities = ein_possibilities_open_initials(possibilities, &rules[0], rules.size());

    puzzle->reset();
    verHints->reset(rules);
    horHints->reset(rules);
    watch->reset();
}

void Game::newGame()
{
    genPuzzle();
    resetVisuals();
}

void Game::restart()
{
    ein_solved_puzzle_free(solvedPuzzle);
    solvedPuzzle = ein_solved_puzzle_clone(savedSolvedPuzzle);

    rules = savedRules;
    
    resetVisuals();
    hinted = true;
}

void Game::run(Config* config, TopScores *top_scores)
{
    Area area = Area(screen);
    Font btnFont(L"laudcn2.ttf", 14);

    WinCommand winCmd(&area, watch, this, config, top_scores);
    FailCommand failCmd(&area, this);
    puzzle->setCommands(&winCmd, &failCmd);

    area.add(puzzle, false);
    area.add(verHints, false);
    area.add(horHints, false);

    area.add(watch, false);

    watch->start();
    area.run();
}
*/

pub fn game_run(surface: Rc<Surface>, game: Rc<RefCell<GamePrivate>>, storage: Rc<RefCell<Storage>>) -> Result<bool> {
    let game_widget = new_game_widget(storage.clone(), game.clone(), get_messages())?;
    game.borrow_mut().start();
    let screen_rect = Rect::new(0, 0, 800, 600);
    let result = main_loop(&surface, screen_rect, &*game_widget)?;
    Ok(result.is_none())
}
