#include <stdlib.h>
#include <iostream>
#include <SDL.h>
#include <SDL_main.h>
#include <SDL_ttf.h>
#include "exceptions.h"
#include "utils.h"
#include "storage.h"
#include "unicode.h"
#include "messages.h"
#include "sound.h"

void menu(Screen *screen);

void initScreen(Screen *screen)
{
    if (TTF_Init())
        throw Exception(L"Error initializing font engine");

    screen->setMode(VideoMode(800, 600, 24, 
                getStorage()->get(L"fullscreen", 1) != 0));
}

extern "C" void initAudio()
{
    sound = new Sound();
    sound->setVolume((float)getStorage()->get(L"volume", 20) / 100.0f);
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

extern "C" void mainpp()
{
    Screen screen;
    initScreen(&screen);
    menu(&screen);
    getStorage()->flush();
}
