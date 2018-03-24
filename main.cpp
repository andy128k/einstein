#include <stdlib.h>
#include <iostream>
#include <SDL.h>
#include <SDL_main.h>
#include "exceptions.h"
#include "screen.h"
#include "utils.h"
#include "unicode.h"
#include "messages.h"
#include "sound.h"
#include "config.h"
#include "resources.h"

extern "C" void initAudio(int volume)
{
    sound = new Sound();
    sound->setVolume(volume / 100.0f);
}

extern "C" void loadResources()
{
    StringList dirs;

    dirs.push_back(PREFIX L"/share/einstein/res");
    dirs.push_back(fromMbcs(getenv("HOME")) + L"/.einstein/res");

    dirs.push_back(L"res");
    dirs.push_back(L".");
    resources = new ResourcesCollection(dirs);
    msg.load();
}

extern "C" int ein_menu(SDL_Surface *surface_ptr, Config *config);

extern "C" Screen *new_screen(int fullscreen) {
    return new Screen(fullscreen);
}

extern "C" void delete_screen(Screen *screen) {
    delete screen;
}

extern "C" void mainpp(int fullscreen, Config *config)
{
    Screen *screen = new_screen(fullscreen);

    int quit = ein_menu(screen->screen, config);
    if (quit) {
        exit(0);
    }

    delete_screen(screen);
}
