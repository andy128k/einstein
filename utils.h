#ifndef __UTILS_H__
#define __UTILS_H__

#include <SDL.h>

extern "C" SDL_Surface* adjust_brightness(SDL_Surface *image, double k);
extern "C" void adjust_brightness_pixel(SDL_Surface *image, int x, int y, double k);
void getPixel(SDL_Surface *surface, int x, int y, Uint8 *r, Uint8 *g, Uint8 *b);
void setPixel(SDL_Surface *s, int x, int y, int r, int g, int b);

#endif
