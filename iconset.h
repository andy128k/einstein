#ifndef __ICONSET_H__
#define __ICONSET_H__


#include <SDL.h>


class IconSet
{
    private:
        SDL_Surface *emptyFieldIcon, *emptyHintIcon;
    
    public:
        IconSet();
        virtual ~IconSet();

    public:
        SDL_Surface* getEmptyFieldIcon() { return emptyFieldIcon; };
        SDL_Surface* getEmptyHintIcon() { return emptyHintIcon; };
};


#endif

