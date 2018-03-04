#include <string.h>
#include "iconset.h"
#include "utils.h"


IconSet::IconSet()
{
    emptyFieldIcon = loadImage(L"tile.bmp");
    emptyHintIcon = loadImage(L"hint-tile.bmp");
}

IconSet::~IconSet()
{
    SDL_FreeSurface(emptyFieldIcon);
    SDL_FreeSurface(emptyHintIcon);
}
