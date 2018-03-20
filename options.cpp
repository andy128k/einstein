#include "options.h"
#include "messages.h"
#include "sound.h"

extern "C" int ein_show_options_window(SDL_Surface*, Config*);

void showOptionsWindow(Area *parentArea, Config *config)
{
    int quit = ein_show_options_window(parentArea->screen->screen, config);
    if (quit) {
        exit(0);
    }
}
