#include <SDL.h>
#include "screen.h"
#include "exceptions.h"
#include "unicode.h"


Screen::Screen(bool fullscreen)
{
    screen = NULL;
    regionsList = NULL;
    maxRegionsList = 0;

    setMode(VideoMode(800, 600, 24, fullscreen));
}

Screen::~Screen()
{
    if (regionsList) free(regionsList);
}


void Screen::setMode(const VideoMode& mode)
{
    fullScreen = mode.isFullScreen();

    int flags = SDL_SWSURFACE /*| SDL_OPENGL*/;
    if (fullScreen)
        flags = flags | SDL_FULLSCREEN;
    screen = SDL_SetVideoMode(mode.getWidth(), mode.getHeight(), mode.getBpp(), flags);
    if (! screen)
        throw Exception(L"Couldn't set video mode: " + 
                fromMbcs((SDL_GetError())));
}


int Screen::getWidth() const
{
    if (screen) 
        return screen->w;
    else 
        throw Exception(L"No video mode selected"); 
}


int Screen::getHeight() const
{
    if (screen) 
        return screen->h;
    else 
        throw Exception(L"No video mode selected");
}

void Screen::flush()
{
    if (! regions.size()) return;
    
    if (! regionsList) {
        regionsList = (SDL_Rect*)malloc(sizeof(SDL_Rect) * regions.size());
        if (! regionsList) {
            regions.clear();
            throw Exception(L"Error allocating regions buffer");
        }
        maxRegionsList = regions.size();
    } else {
        if (maxRegionsList < (int)regions.size()) {
            SDL_Rect *r = (SDL_Rect*)realloc(regionsList, 
                    sizeof(SDL_Rect) * regions.size());
            if (! r) {
                regions.clear();
                free(regionsList);
                throw Exception(L"Error incrementing regions buffer");
            }
            regionsList = r;
            maxRegionsList = regions.size();
        }
    }

    int j = 0;
    for (std::list<SDL_Rect>::iterator i = regions.begin();
            i != regions.end(); i++, j++)
        regionsList[j] = *i;

    SDL_UpdateRects(screen, regions.size(), regionsList);
    regions.clear();
}


void Screen::addRegionToUpdate(int x, int y, int w, int h)
{
    if (((x >= getWidth()) || (y >= getHeight())) || (0 >= w) || (0 >= h))
        return;
    if ((x + w < 0) || (y + h < 0))
        return;
    if (x + w > getWidth())
        w = getWidth() - x;
    if (y + h > getHeight())
        h = getHeight() - y;
    if (0 > x) {
        w = w + x;
        x = 0;
    }
    if (0 > y) {
        h = h + y;
        y = 0;
    }
    SDL_Rect r = { x, y, w, h };
    regions.push_back(r);
}


void Screen::setPixel(int x, int y, int r, int g, int b)
{
    SDL_LockSurface(screen);
    int bpp = screen->format->BytesPerPixel;
    Uint32 pixel = SDL_MapRGB(screen->format, r, g, b);
    /* Here p is the address to the pixel we want to set */
    Uint8 *p = (Uint8*)screen->pixels + y * screen->pitch + x * bpp;

    switch(bpp) {
        case 1:
            *p = pixel;
            break;

        case 2:
            *(Uint16 *)p = pixel;
            break;

        case 3:
            if(SDL_BYTEORDER == SDL_BIG_ENDIAN) {
                p[0] = (pixel >> 16) & 0xff;
                p[1] = (pixel >> 8) & 0xff;
                p[2] = pixel & 0xff;
            } else {
                p[0] = pixel & 0xff;
                p[1] = (pixel >> 8) & 0xff;
                p[2] = (pixel >> 16) & 0xff;
            }
            break;

        case 4:
            *(Uint32 *)p = pixel;
            break;
    }
    SDL_UnlockSurface(screen);
}


void Screen::draw(int x, int y, SDL_Surface *tile)
{
    SDL_Rect src = { 0, 0, tile->w, tile->h };
    SDL_Rect dst = { x, y, tile->w, tile->h };
    SDL_BlitSurface(tile, &src, screen, &dst);
}

SDL_Surface* Screen::createSubimage(int x, int y, int width, int height)
{
    SDL_Surface *s = SDL_CreateRGBSurface(SDL_HWSURFACE | SDL_SRCCOLORKEY,
            width, height, screen->format->BitsPerPixel,
            screen->format->Rmask, screen->format->Gmask,
            screen->format->Bmask, screen->format->Amask);
    if (! s)
        throw Exception(L"Error creating buffer surface");
    SDL_Rect src = { x, y, width, height };
    SDL_Rect dst = { 0, 0, width, height };
    SDL_BlitSurface(screen, &src, s, &dst);
    return s;
}
