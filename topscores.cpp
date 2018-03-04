#include "topscores.h"
#include "utils.h"
#include "font.h"
#include "convert.h"
#include "messages.h"

class ScoresWindow: public Window
{
    public:
        ScoresWindow(Screen *screen, int x, int y, TopScores *scores, int highlight);
};


ScoresWindow::ScoresWindow(Screen *screen, int x, int y, TopScores *scores, int highlight)
    : Window(screen, x, y, 320, 350, L"blue.bmp")
{
    Font titleFont(L"nova.ttf", 26);
    Font entryFont(L"laudcn2.ttf", 14);
    Font timeFont(L"luximb.ttf", 14);
    
    std::wstring txt = msg(L"topScores");
    int w = titleFont.getWidth(txt);
    titleFont.draw(background, (320 - w) / 2, 15, 255,255,0, true, txt);

    int no = 1;
    int pos = 70;
    int len = ein_topscores_get_count(scores);
    for (int i = 0; i < len; i++)
    {
        std::wstring name = fromUtf8(ein_topscores_get_name(scores, i));
        int score = ein_topscores_get_score(scores, i);

        std::wstring s(toString(no) + L".");
        int w = entryFont.getWidth(s);
        int c = ((no - 1) == highlight) ? 0 : 255;
        entryFont.draw(background, 30 - w, pos, 255,255,c, true, s);
        SDL_Rect rect = { 40, pos-20, 180, 40 };
        SDL_SetClipRect(background, &rect);
        entryFont.draw(background, 40, pos, 255,255,c, true, name);
        SDL_SetClipRect(background, NULL);
        s = secToStr(score);
        w = timeFont.getWidth(s);
        timeFont.draw(background, 305-w, pos, 255,255,c, true, s);
        pos += 20;
        no++;
    }
}


void showScoresWindow(Area *parentArea, TopScores *scores, int highlight)
{
    Screen *screen = parentArea->screen;

    Area area(screen);

    Font font(L"laudcn2.ttf", 16);
    area.add(parentArea);
    area.add(new ScoresWindow(screen, 240, 125, scores, highlight));
    ExitCommand exitCmd(area);
    area.add(new Button(screen, 348, 430, 90, 25, &font, 255,255,0, L"blue.bmp", 
                msg(L"ok"), &exitCmd));
    area.add(new KeyAccel(screen, SDLK_ESCAPE, &exitCmd));
    area.run();
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
