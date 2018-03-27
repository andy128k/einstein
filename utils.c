#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <SDL.h>

void setPixel(SDL_Surface *s, int x, int y, int r, int g, int b)
{
    int bpp = s->format->BytesPerPixel;
    Uint32 pixel = SDL_MapRGB(s->format, r, g, b);
    /* Here p is the address to the pixel we want to set */
    Uint8 *p = (Uint8*)s->pixels + y * s->pitch + x * bpp;

    switch (bpp) {
        case 1:
            *p = pixel;
            break;
        case 2:
            *(Uint16 *)p = pixel;
            break;
        case 3:
            if (SDL_BYTEORDER == SDL_BIG_ENDIAN) {
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
}


void getPixel(SDL_Surface *surface, int x, int y, 
        Uint8 *r, Uint8 *g, Uint8 *b)
{
    int bpp = surface->format->BytesPerPixel;
    /* Here p is the address to the pixel we want to retrieve */
    Uint8 *p = (Uint8 *)surface->pixels + y * surface->pitch + x * bpp;

    Uint32 pixel;
    switch (bpp) {
        case 1: pixel = *p;  break;
        case 2: pixel = *(Uint16 *)p; break;
        case 3:
            if (SDL_BYTEORDER == SDL_BIG_ENDIAN)
                pixel = p[0] << 16 | p[1] << 8 | p[2];
            else
                pixel = p[0] | p[1] << 8 | p[2] << 16;
            break;
        case 4: pixel = *(Uint32 *)p; break;
        default: pixel = 0;       /* shouldn't happen, but avoids warnings */
    }
    SDL_GetRGB(pixel, surface->format, r, g, b);
}


static int gammaTable[256];
static double lastGamma = -1.0;


void adjust_brightness_pixel(SDL_Surface *image, int x, int y, double k)
{
    if (lastGamma != k) {
        for (int i = 0; i <= 255; i++) {
            gammaTable[i] = (int)(255.0 * pow((double)i / 255.0, 1.0 / k) + 0.5);
            if (gammaTable[i] > 255)
                gammaTable[i] = 255;
        }
        lastGamma = k;
    }
    
    Uint8 r, g, b;
    getPixel(image, x, y, &r, &g, &b);
    setPixel(image, x, y, gammaTable[r], gammaTable[g], gammaTable[b]);
}


SDL_Surface* adjust_brightness(SDL_Surface *image, double k)
{
    if (lastGamma != k) {
        for (int i = 0; i <= 255; i++) {
            gammaTable[i] = (int)(255.0 * pow((double)i / 255.0, 1.0 / k) + 0.5);
            if (gammaTable[i] > 255)
                gammaTable[i] = 255;
        }
        lastGamma = k;
    }
    
    SDL_Surface *s = SDL_DisplayFormat(image);
    if (! s) {
        fprintf(stderr, "Error converting image to display format.\n");
        abort();
    }
    
    SDL_LockSurface(s);
    
    Uint8 r, g, b;
    for (int j = 0; j < s->h; j++)
        for (int i = 0; i < s->w; i++) {
            getPixel(s, i, j, &r, &g, &b);
            setPixel(s, i, j, gammaTable[r], gammaTable[g], gammaTable[b]);
        }
    
    SDL_UnlockSurface(s);

    return s;
}
