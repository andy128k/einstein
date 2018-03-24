#ifndef __UTILS_H__
#define __UTILS_H__

#include <SDL.h>
#include <string>
#ifdef WIN32
#include <sys/time.h>
#endif
#include <iostream>



SDL_Surface* loadImage(const std::wstring &name, bool transparent=false);
extern "C" SDL_Surface* adjust_brightness(SDL_Surface *image, double k, int transparent=false);
int gettimeofday(struct timeval* tp);
bool isInRect(int evX, int evY, int x, int y, int w, int h);
std::wstring numToStr(int no);
extern "C" void adjust_brightness_pixel(SDL_Surface *image, int x, int y, double k);
std::wstring secToStr(int time);
int getCornerPixel(SDL_Surface *surface);
void getPixel(SDL_Surface *surface, int x, int y, 
        Uint8 *r, Uint8 *g, Uint8 *b);
void setPixel(SDL_Surface *s, int x, int y, int r, int g, int b);
void drawBevel(SDL_Surface *s, int left, int top, int width, int height,
        bool raised, int size);
void ensureDirExists(const std::wstring &fileName);
int readInt(std::istream &stream);
std::wstring readString(std::istream &stream);
void writeInt(std::ostream &stream, int value);
void writeString(std::ostream &stream, const std::wstring &value);

/// Read 4-bytes integer from memory.
int readInt(unsigned char *buffer);


#endif

