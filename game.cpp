#include "utils.h"
#include "widgets.h"
#include "puzzle.h"
#include "verthints.h"
#include "horhints.h"
#include "widgets.h"
#include "font.h"
#include "topscores.h"
#include "opensave.h"
#include "options.h"
#include "game.h"
#include "messages.h"
#include "sound.h"
#include "descr.h"
#include "unicode.h"



//////////////////////////////////////////////////////////////////////////
// DUMMY
static void savePuzzle(SolvedPuzzle *puzzle, std::ostream &stream) {}
static void loadPuzzle(SolvedPuzzle *puzzle, std::istream &stream) {}
static void saveRules(RulesArr &rules, std::ostream &stream){}
static void loadRules(RulesArr &rules, std::istream &stream){}
//////////////////////////////////////////////////////////////////////////


class GameBackground: public Widget
{
    public:
        GameBackground(Screen *screen): Widget(screen) {}
        virtual void draw();
};


void GameBackground::draw()
{
    // draw background
    drawWallpaper(screen, L"rain.bmp");

    // draw title
    SDL_Surface *tile = loadImage(L"title.bmp");
    screen->draw(8, 10, tile);
    SDL_FreeSurface(tile);
    
    Font titleFont(L"nova.ttf", 28);
    titleFont.draw(screen->getSurface(), 20, 20, 255,255,0, true, 
            msg(L"einsteinPuzzle"));
    
    screen->addRegionToUpdate(0, 0, screen->getWidth(), screen->getHeight());
}


class Watch: public TimerHandler, public Widget
{
    private:
        Uint32 lastRun;
        Uint32 elapsed;
        bool stoped;
        int lastUpdate;
        Font *font;
    
    public:
        Watch(Screen *screen);
        Watch(Screen *screen, std::istream &stream);
        virtual ~Watch();

    public:
        virtual void onTimer();
        void stop();
        void start();
        virtual void draw();
        int getElapsed() { return elapsed; };
        void save(std::ostream &stream);
        void reset();
};


Watch::Watch(Screen *screen)
    : Widget(screen)
{
    lastRun = elapsed = lastUpdate = 0;
    stop();
    font = new Font(L"luximb.ttf", 16);
}

Watch::Watch(Screen *screen, std::istream &stream)
    : Widget(screen)
{
    elapsed = readInt(stream);
    lastUpdate = 0;
    stop();
    font = new Font(L"luximb.ttf", 16);
}

Watch::~Watch()
{
    delete font;
}

void Watch::onTimer()
{
    if (stoped)
        return;
    
    Uint32 now = SDL_GetTicks();
    elapsed += now - lastRun;
    lastRun = now;

    int seconds = elapsed / 1000;
    if (seconds != lastUpdate)
        draw();
}

void Watch::stop()
{
    stoped = true;
}

void Watch::start()
{
    stoped = false;
    lastRun = SDL_GetTicks();
}

void Watch::draw()
{
    int time = elapsed / 1000;
    std::wstring s = secToStr(time);
    
    int x = 700;
    int y = 24;
    int w, h;
    font->getSize(s, w, h);
    SDL_Rect rect = { x-2, y-2, w+4, h+4 };
    SDL_FillRect(screen->getSurface(), &rect, 
            SDL_MapRGB(screen->getSurface()->format, 0, 0, 255));
    font->draw(screen->getSurface(), x, y, 255,255,255, true, s);
    screen->addRegionToUpdate(x-2, y-2, w+4, h+4);
    
    lastUpdate = time;
}

void Watch::save(std::ostream &stream)
{
    writeInt(stream, elapsed);
}

void Watch::reset()
{
    elapsed = lastUpdate = 0;
    lastRun = SDL_GetTicks();
}


class PauseGameCommand: public Command
{
    private:
        Area *gameArea;
        Watch *watch;
        Widget *background;

    public:
        PauseGameCommand(Area *a, Watch *w, Widget *bg) { 
            gameArea = a; 
            watch = w;
            background = bg;
        };
        
        virtual void doAction() {
            watch->stop();
            Screen *screen = gameArea->screen;
            Area area(screen);
            area.add(background, false);
            Font font(L"laudcn2.ttf", 16);
            area.add(new Window(screen, 280, 275, 240, 50, L"greenpattern.bmp", 6));
            area.add(new Label(screen, &font, 280, 275, 240, 50, Label::ALIGN_CENTER,
                Label::ALIGN_MIDDLE, 255,255,0, msg(L"paused")));
            area.add(new AnyKeyAccel(screen));
            area.run();
            sound->play(L"click.wav");
            gameArea->updateMouse();
            gameArea->draw();
            watch->start();
        };
};


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
            showMessageWindow(gameArea->screen, gameArea, L"marble1.bmp", 
                    500, 70, &font, 255,0,0, msg(L"won"));
            gameArea->draw();

            int score = watch->getElapsed() / 1000;
            int pos = -1;
            if (! game->isHinted()) {
                if (ein_topscores_is_deserving(top_scores, score)) {
                    const char* lastname = ein_config_get_last_name(config);
                    if (lastname == NULL) {
                        lastname = "anonymous";
                    }
                    auto wlastname = fromUtf8(lastname);
                    std::string name = toUtf8(enterNameDialog(gameArea, wlastname));

                    ein_config_set_last_name(config, name.c_str());
                    pos = ein_topscores_add(top_scores, name.c_str(), score);
                }
            }
            showScoresWindow(gameArea, top_scores, pos);
            gameArea->finishEventLoop();
        };
};

class OkDlgCommand: public Command
{
    private:
        bool &res;
        Area *area;

    public:
        OkDlgCommand(Area *a, bool &r): res(r) { 
            area = a; 
        };
        
        virtual void doAction() { 
            res = true; 
            area->finishEventLoop();
        };
};

class FailCommand: public Command
{
    private:
        Area *gameArea;
        Game *game;

    public:
        FailCommand(Area *a, Game *g) { gameArea = a;  game = g; };
        
        virtual void doAction() {
            sound->play(L"glasbk2.wav");
            bool restart = false;
            bool newGame = false;
            Font font(L"laudcn2.ttf", 24);
            Font btnFont(L"laudcn2.ttf", 14);
            Screen *screen = gameArea->screen;
            Area area = Area(screen);
            area.add(gameArea);
            area.add(new Window(screen, 220, 240, 360, 140, L"redpattern.bmp", 6));
            area.add(new Label(screen, &font, 250, 230, 300, 100, Label::ALIGN_CENTER,
                        Label::ALIGN_MIDDLE, 255,255,0, msg(L"loose")));
            OkDlgCommand newGameCmd(&area, newGame);
            area.add(new Button(screen, 250, 340, 90, 25, &btnFont, 255,255,0, 
                        L"redpattern.bmp", msg(L"startNew"), &newGameCmd));
            OkDlgCommand restartCmd(&area, restart);
            area.add(new Button(screen, 350, 340, 90, 25, &btnFont, 255,255,0, 
                        L"redpattern.bmp", msg(L"tryAgain"), &restartCmd));
            ExitCommand exitCmd(area);
            area.add(new Button(screen, 450, 340, 90, 25, &btnFont, 255,255,0, 
                        L"redpattern.bmp", msg(L"exit"), &exitCmd));
            area.run();
            if (restart || newGame) {
                if (newGame)
                    game->newGame();
                else
                    game->restart();
                gameArea->draw();
                gameArea->updateMouse();
            } else
                gameArea->finishEventLoop();
        };
};


class GameOptionsCommand: public Command
{
    private:
        Area *gameArea;
        Config *config;

    public:
        GameOptionsCommand(Area *a, Config *config) { 
            gameArea = a; 
            this->config = config;
        };
        
        virtual void doAction() {
            showOptionsWindow(gameArea, config);
            gameArea->updateMouse();
            gameArea->draw();
        };
};


class HelpCommand: public Command
{
    private:
        Area *gameArea;
        Watch *watch;
        Widget *background;

    public:
        HelpCommand(Area *a, Watch *w, Widget *b) { 
            gameArea = a;
            watch = w;
            background = b;
        };
        
        virtual void doAction() {
            watch->stop();
            Area area(gameArea->screen);
            area.add(background, false);
            area.draw();
            showDescription(&area);
            gameArea->updateMouse();
            gameArea->draw();
            watch->start();
        };
};



Game::Game(Screen *screen)
    : screen(screen)
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

Game::~Game()
{
    delete watch;
    ein_possibilities_free(possibilities);
    delete verHints;
    delete horHints;
    delete puzzle;
    deleteRules();
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

void Game::deleteRules()
{
    for (RulesArr::iterator i = rules.begin(); i != rules.end(); i++) {
        ein_rule_free(*i);
    }
    rules.clear();
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
    
    if (rules.size() > 0)
        deleteRules();

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

#define BUTTON(screen, x, y, text, cmd) \
    area.add(new Button(screen, x, y, 94, 30, &btnFont, 255,255,0, \
                L"btn.bmp", msg(text), false, cmd));

extern "C" int ein_game_run(SDL_Surface *surface_ptr, Config* config, TopScores *top_scores);

void Game::run(Config* config, TopScores *top_scores)
{
    int quit = ein_game_run(screen->screen, config, top_scores);
    if (quit) {
        exit(0);
    }
}
