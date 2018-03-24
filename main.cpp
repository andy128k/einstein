#include <stdlib.h>
#include <iostream>
#include <SDL.h>
#include <SDL_main.h>
#include "exceptions.h"
#include "utils.h"
#include "unicode.h"
#include "messages.h"
#include "sound.h"
#include "config.h"
#include "resources.h"

void menu(Screen *screen, Config *config);

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

extern "C" void mainpp(int fullscreen, Config *config)
{
    Screen screen(fullscreen);
    menu(&screen, config);
}
