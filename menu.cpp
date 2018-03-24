#include <vector>
#include "utils.h"
#include "widgets.h"
#include "descr.h"
#include "messages.h"
#include "config.h"



class MenuBackground: public Widget
{
    public:
        MenuBackground(Screen *screen): Widget(screen) {}
        virtual void draw();
};


void MenuBackground::draw()
{
    SDL_Surface *title = loadImage(L"nova.bmp");
    screen->draw(0, 0, title);
    SDL_FreeSurface(title);
    Font font(L"nova.ttf", 28);
    std::wstring s(msg(L"einsteinFlowix"));
    int width = font.getWidth(s);
    font.draw(screen->getSurface(), (screen->getWidth() - width) / 2, 30, 255,255,255, true, s);
    Font urlFont(L"luximb.ttf", 16);
    s = L"http://games.flowix.com";
    width = urlFont.getWidth(s);
    urlFont.draw(screen->getSurface(), (screen->getWidth() - width) / 2, 60, 255,255,0, true, s);
    screen->addRegionToUpdate(0, 0, screen->getWidth(), screen->getHeight());
}

extern "C" int ein_game_run(SDL_Surface *surface_ptr, Config* config);
extern "C" int ein_game_load(SDL_Surface *surface_ptr, Config* config);

class NewGameCommand: public Command
{
    private:
        Area *area;
        Config *config;
    
    public:
        NewGameCommand(Area *a, Config *config) { area = a; this->config = config; };

        virtual void doAction() {
            Screen *screen = area->screen;

            int quit = ein_game_run(screen->screen, config);
            if (quit) {
                exit(0);
            }

            area->updateMouse();
            area->draw();
        };
};


class LoadGameCommand: public Command
{
    private:
        Area *area;
        Config *config;
    
    public:
        LoadGameCommand(Area *a, Config *config) { area = a; this->config = config; };

        virtual void doAction() {
            Screen *screen = area->screen;
            int quit = ein_game_load(screen->screen, config);
            if (quit) {
                exit(0);
            }
            area->updateMouse();
            area->draw();
        };
};


extern "C" int ein_show_scores(SDL_Surface *surface, Config *config);

class TopScoresCommand: public Command
{
    private:
        Area *area;
        Config *config;
    
    public:
        TopScoresCommand(Area *a, Config *config) { area = a; this->config = config; };

        virtual void doAction() {
            int quit = ein_show_scores(area->screen->screen, config);
            if (quit) {
                exit(0);
            }

            area->updateMouse();
            area->draw();
        };
};


class RulesCommand: public Command
{
    private:
        Area *area;
    
    public:
        RulesCommand(Area *a) { area = a; };

        virtual void doAction() {
            showDescription(area);
            area->updateMouse();
            area->draw();
        };
};

extern "C" int ein_show_options_window(SDL_Surface*, Config*);

class OptionsCommand: public Command
{
    private:
        Area *area;
        Config *config;
    
    public:
        OptionsCommand(Area *a, Config *config) { area = a; this->config = config; };

        virtual void doAction() {
            int quit = ein_show_options_window(area->screen->screen, config);
            if (quit) {
                exit(0);
            }

            area->updateMouse();
            area->draw();
        };
};


extern "C" int ein_show_about(SDL_Surface *surface);


class AboutCommand: public Command
{
    private:
        Area *parentArea;
    
    public:
        AboutCommand(Area *a) { parentArea = a; };

        virtual void doAction() {
            Screen *screen = parentArea->screen;

            int quit = ein_show_about(screen->screen);
            if (quit) {
                exit(0);
            }

            parentArea->updateMouse();
            parentArea->draw();
        };
};


static Button* menuButton(Screen *screen, int y, Font *font, const std::wstring &text, 
        Command *cmd=NULL)
{
    return new Button(screen, 550, y, 220, 30, font, 0,240,240, 30,255,255, text, cmd);
}


void menu(Screen *screen, Config *config)
{
    Area area(screen);
    Font font(L"laudcn2.ttf", 20);

    area.add(new MenuBackground(screen));
    area.draw();
        
    NewGameCommand newGameCmd(&area, config);
    area.add(menuButton(screen, 340, &font, msg(L"newGame"), &newGameCmd));
    LoadGameCommand loadGameCmd(&area, config);
    area.add(menuButton(screen, 370, &font, msg(L"loadGame"), &loadGameCmd));
    TopScoresCommand topScoresCmd(&area, config);
    area.add(menuButton(screen, 400, &font, msg(L"topScores"), &topScoresCmd));
    RulesCommand rulesCmd(&area);
    area.add(menuButton(screen, 430, &font, msg(L"rules"), &rulesCmd));
    OptionsCommand optionsCmd(&area, config);
    area.add(menuButton(screen, 460, &font, msg(L"options"), &optionsCmd));
    AboutCommand aboutCmd(&area);
    area.add(menuButton(screen, 490, &font, msg(L"about"), &aboutCmd));
    ExitCommand exitMenuCmd(area);
    area.add(menuButton(screen, 520, &font, msg(L"exit"), &exitMenuCmd));
    area.add(new KeyAccel(screen, SDLK_ESCAPE, &exitMenuCmd));
    
    area.draw();
    screen->addRegionToUpdate(0, 0, screen->getWidth(), screen->getHeight());
    screen->flush();

    area.run();
}
