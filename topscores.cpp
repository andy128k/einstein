#include "topscores.h"
#include "utils.h"
#include "font.h"
#include "convert.h"
#include "messages.h"

extern "C" int ein_show_scores(SDL_Surface *surface, TopScores *scores, int highlight);

void showScoresWindow(Area *parentArea, TopScores *scores, int highlight)
{
    int quit = ein_show_scores(parentArea->screen->screen, scores, highlight);
    if (quit) {
        exit(0);
    }
}


std::wstring enterNameDialog(Area *parentArea, std::wstring &name)
{
    Screen *screen = parentArea->screen;

    Area area(screen);
    
    Font font(L"laudcn2.ttf", 16);
    area.add(parentArea);
    area.add(new Window(screen, 170, 280, 460, 100, L"blue.bmp"));
    area.add(new Label(screen, &font, 180, 300, 255,255,0, msg(L"enterName")));
    area.add(new InputField(screen, 350, 300, 270, 26, L"blue.bmp", name, 20,  
                255,255,0,  &font));
    ExitCommand exitCmd(area);
    area.add(new Button(screen, 348, 340, 90, 25, &font, 255,255,0, L"blue.bmp", 
                msg(L"ok"), &exitCmd));
    area.add(new KeyAccel(screen, SDLK_ESCAPE, &exitCmd));
    area.add(new KeyAccel(screen, SDLK_RETURN, &exitCmd));
    area.run();
    return name;
}
