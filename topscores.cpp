#include "topscores.h"
#include "utils.h"
#include "font.h"
#include "convert.h"
#include "messages.h"

extern "C" int ein_show_scores(SDL_Surface *surface, TopScores *scores, int highlight);
extern "C" int ein_ask_player_name(SDL_Surface *surface, const char *default_name, char *name_ptr);

void showScoresWindow(Area *parentArea, TopScores *scores, int highlight)
{
    int quit = ein_show_scores(parentArea->screen->screen, scores, highlight);
    if (quit) {
        exit(0);
    }
}


std::wstring enterNameDialog(Area *parentArea, std::wstring &wdefault_name)
{
    std::string default_name = toMbcs(wdefault_name);

    char name[200];
    switch (ein_ask_player_name(parentArea->screen->screen, default_name.c_str(), name)) {
        case 0:
            return fromMbcs(name);
        default:
            exit(0);
    }
}
