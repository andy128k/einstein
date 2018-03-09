#include "descr.h"

extern "C" int ein_show_description(SDL_Surface *surface_ptr);

void showDescription(Area *parentArea)
{
    int quit = ein_show_description(parentArea->screen->screen);
    if (quit) {
        exit(0);
    }
}
