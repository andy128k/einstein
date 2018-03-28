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
use ui::widget::container::Container;
use ui::widget::window::Window;
use ui::widget::title::Title;
use ui::widget::button::*;
use ui::widget::game_button::new_game_button;
use ui::widget::image::Image;
use ui::widget::label::Label;
use ui::utils::{load_image, draw_text, HorizontalAlign, VerticalAlign, adjust_brightness};
use ui::main_loop::main_loop;
use ui::component::dialog::DialogResult;
use ui::component::puzzle::puzzle::new_puzzle_widget;
use ui::component::puzzle::horizontal_rules::HorizontalRules;
use ui::component::puzzle::vertical_rules::VerticalRules;
use ui::component::watch::Watch;
use ui::component::rules_dialog::show_description;
use ui::component::save_dialog::save_game;
use ui::component::options_dialog::show_options_window;
use ui::component::pause_dialog::*;
use ui::component::failure_dialog::{show_failure_dialog, Choice};
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

    // VertHints *verHints;
    // HorHints *horHints;
    // IconSet iconSet;
    // Puzzle *puzzle;
    // Watch *watch;
    // bool hinted;
    // SolvedPuzzle *savedSolvedPuzzle;
    // RulesArr savedRules;
    // Screen *screen;
}

//////////////////////////////////////////////////////////////////////////
// DUMMY
// fn savePuzzle(puzzle: &SolvedPuzzle) {}
// fn loadPuzzle() -> SolvedPuzzle {}
// fn saveRules(rules: &[Rule]) {}
// fn loadRules() -> Vec<Rule> {}
//////////////////////////////////////////////////////////////////////////

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
        })))
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

pub fn new_game_widget<FnHelp, FnOpts, FnSave, FnPause>(surface: Rc<Surface>, state: Rc<RefCell<GamePrivate>>, show_help: FnHelp, show_options: FnOpts, save: FnSave, show_pause: FnPause) -> Result<Container<()>>
where
    FnHelp: Fn() -> bool + 'static,
    FnOpts: Fn() -> bool + 'static,
    FnSave: Fn() -> bool + 'static,
    FnPause: Fn() -> bool + 'static,
{
    let screen_rect = Rect::new(0, 0, 800, 600);

    let mut container = Container::new(screen_rect, ());

    container.add(Box::new(Image::new(screen_rect, RAIN_TILE)?));
    container.add(Box::new(Image::new(Rect::new(8, 10, 783, 47), TITLE_BG)?));
    container.add(Box::new(Title {
        text: "Einstein Puzzle".to_string(), // i18n msg(L"einsteinPuzzle")
        rect: Rect::new(20, 10, 500, 47),
    }));

    container.add(Box::new(Watch::new(state.clone())));
    container.add(Box::new({
        let surface2 = surface.clone();
        let surface3 = surface.clone();
        new_puzzle_widget(state.clone(),
            Rc::new(move || {
        /*
        class WinCommand: public Command
        {
            private:
                Area *gameArea;
                Watch *watch;
                Game *game;
                TopScores* top_scores;
                Config* config;

            public:
                WinCommand(Area *a, Watch *w, Game *g, Config* config, TopScores* top_scores) { 
                    gameArea = a; 
                    watch = w;
                    game = g;
                    this->config = config;
                    this->top_scores = top_scores;
                };

                virtual void doAction() {
                    sound->play(L"applause.wav");
                    watch->stop();
                    Font font(L"laudcn2.ttf", 20);
                    showMessageWindow(gameArea->screen, gameArea, L"marble1.bmp", 500, 70, &font, 255,0,0, msg(L"won"));
                    gameArea->draw();

                    int score = watch->getElapsed() / 1000;
                    int pos = -1;
                    if (! game->isHinted()) {
                        if (storage.scores.is_deserving(score)) {
                            const char* lastname = ein_config_get_last_name(config);
                            if (lastname == NULL) {
                                lastname = "anonymous";
                            }
                            auto wlastname = fromUtf8(lastname);

                            match ask_player_name(&surface, &last_name).unwrap() {
                                DialogResult::Ok(name) => {
                                    storage.last_name = Some(name);
                                    pos = storage.scores.add_score_entry(storage::Score { name, score });
                                },
                                DialogResult::Cancel => {},
                                DialogResult::Quit =>
                                    exit(0),
                            }
                        }
                    }

                    let quit = show_scores(&surface, scores, pos)?;
                    if quit {
                        exit(0);
                    }

                    gameArea->finishEventLoop();
                };
        */
            }),
            Rc::new(move || {
                // sound->play(L"glasbk2.wav");
                match show_failure_dialog(&*surface2).unwrap() {
                    DialogResult::Ok(Choice::StartNew) => {
                        // game->newGame();
                        Some(Effect::Terminate)
                    },
                    DialogResult::Ok(Choice::TryAgain) => {

                        // let possibilities = Possibilities::new();
                        // possibilities = ein_possibilities_open_initials(possibilities, &rules[0], rules.size());

                        // rules = savedRules;

                        // hinted = true;
                        Some(Effect::Terminate)
                    },
                    DialogResult::Cancel => Some(Effect::Terminate),
                    DialogResult::Quit => Some(Effect::Quit),
                };
            })
        )?
    }));
    container.add(Box::new(HorizontalRules::new(state.clone())?));
    container.add(Box::new(VerticalRules::new(state.clone())?));

    container.add(Box::new({
        let this_state = Rc::downgrade(&state);
        new_game_button(
            Rect::new(12, 440, 94, 30),
            "Save", // TODO i18n
            None,
            move || {
                let state = this_state.upgrade()?;
                state.borrow_mut().stop();

                let quit = save();
                if quit {
                    return Some(Effect::Quit);
                }

                state.borrow_mut().start();
                Some(Effect::Redraw(vec![screen_rect]))
            }
        )
    }));

    container.add(Box::new({
        let this_state = Rc::downgrade(&state);
        new_game_button(
            Rect::new(119, 400, 94, 30),
            "switch", // TODO i18n
            None,
            move || {
                let state = this_state.upgrade()?;
                state.borrow_mut().toggle_show_excluded();
                Some(Effect::Redraw(vec![screen_rect]))
            }
        )
    }));

    container.add(Box::new(new_game_button(
        Rect::new(226, 400, 94, 30),
        "Exit", // TODO i18n
        Some(Key::Escape),
        || Some(Effect::Terminate)
    )));

    container.add(Box::new({
        let this_state = Rc::downgrade(&state);
        new_game_button(
            Rect::new(226, 440, 94, 30),
            "Help", // TODO i18n
            None,
            move || {
                let state = this_state.upgrade()?;
                state.borrow_mut().stop();

                let quit = show_help();
                if quit {
                    return Some(Effect::Quit);
                }

                state.borrow_mut().start();
                Some(Effect::Redraw(vec![screen_rect]))
            }
        )
    }));

    container.add(Box::new({
        let this_state = Rc::downgrade(&state);
        new_game_button(
            Rect::new(119, 440, 94, 30),
            "Options", // TODO i18n
            None,
            move || {
                let state = this_state.upgrade()?;
                state.borrow_mut().stop();

                let quit = show_options();
                if quit {
                    return Some(Effect::Quit);
                }

                state.borrow_mut().start();
                Some(Effect::Redraw(vec![screen_rect]))
            }
        )
    }));

    container.add(Box::new({
        let this_state = Rc::downgrade(&state);
        new_game_button(
            Rect::new(12, 400, 94, 30),
            "Pause", // TODO i18n
            None,
            move || {
                let state = this_state.upgrade()?;
                state.borrow_mut().stop();

                let quit = show_pause();
                if quit {
                    return Some(Effect::Quit);
                }

                state.borrow_mut().start();
                Some(Effect::Redraw(vec![screen_rect]))
            }
        )
    }));

    Ok(container)
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
    let game_widget = new_game_widget(surface.clone(), game.clone(),
        {
            let surface2 = surface.clone();
            move || {
                show_description(&surface2).expect("No errors")
            }
        },
        {
            let surface2 = surface.clone();
            let storage2 = storage.clone();
            move || {
                show_options_window(&*surface2, &mut storage2.borrow_mut()).expect("No errors")
            }
        },
        {
            let surface2 = surface.clone();
            let storage2 = storage.clone();
            let game2 = game.clone();
            move || {
                save_game(surface2.clone(), &mut storage2.borrow_mut(), &game2.borrow()).expect("No errors");
                false
            }
        },
        {
            let surface2 = surface.clone();
            move || {
                pause(&*surface2).expect("No errors")
            }
        }
    )?;
    game.borrow_mut().start();
    main_loop(&surface, &game_widget)
}
