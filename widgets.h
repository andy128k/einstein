#ifndef __WIDGETS_H__
#define __WIDGETS_H__

#include <string>
#include <list>
#include <set>
#include <SDL.h>
#include "font.h"
#include "screen.h"


class Command
{
    public:
        virtual ~Command() { };
        virtual void doAction() = 0;
};


class Area;


class Widget
{
    protected:
        Area *area;
    
    public:
        Screen *screen;

        Widget(Screen* screen): screen(screen) { };
        virtual ~Widget() { };

    public:
        virtual bool onMouseButtonDown(int /*button*/, int /*x*/, int /*y*/) { return false; };
        virtual bool onMouseButtonUp(int /*button*/, int /*x*/, int /*y*/) { return false; };
        virtual bool onMouseMove(int /*x*/, int /*y*/) { return false; };
        virtual void draw() { };
        virtual void setParent(Area *a) { area = a; };
        virtual bool onKeyDown(SDLKey /*key*/, unsigned char /*ch*/) { return false; };
        virtual bool destroyByArea() { return true; };
};


class Button: public Widget
{
    protected:
        int left, top, width, height;
        SDL_Surface *image, *highlighted;
        bool mouseInside;
        Command *command;
        
    public:
        Button(Screen *screen, int x, int y, const std::wstring &name, Command *cmd=NULL, 
                bool transparent=true);
        Button(Screen *screen, int x, int y, int width, int height, Font *font, 
                int fR, int fG, int fB, int hR, int hG, int hB, 
                const std::wstring &text, Command *cmd=NULL);
        Button(Screen *screen, int x, int y, int width, int height, Font *font, 
                int r, int g, int b, const std::wstring &background, 
                const std::wstring &text, Command *cmd=NULL);
        Button(Screen *screen, int x, int y, int width, int height, Font *font, 
                int r, int g, int b, const std::wstring &background, 
                const std::wstring &text, bool bevel, Command *cmd=NULL);
        virtual ~Button();

    public:
        virtual void draw();
        void getBounds(int &left, int &top, int &width, int &height);
        int getLeft() const { return left; };
        int getTop() const { return top; };
        int getWidth() const { return width; };
        int getHeight() const { return height; };
        virtual bool onMouseButtonDown(int button, int x, int y);
        virtual bool onMouseMove(int x, int y);
        void moveTo(int x, int y) { left = x; top = y; };
};



class KeyAccel: public Widget
{
    protected:
        SDLKey key;
        Command *command;

    public:
        KeyAccel(Screen *screen, SDLKey key, Command *command);
        virtual bool onKeyDown(SDLKey key, unsigned char ch);
};


class TimerHandler
{
    public:
        virtual ~TimerHandler() { };
        virtual void onTimer() = 0;
};


class Area: public Widget
{
    private:
        typedef std::list<Widget*> WidgetsList;
        WidgetsList widgets;
        std::set<Widget*> notManagedWidgets;
        bool terminate;
        Uint32 time;
        TimerHandler *timer;

    public:
        Area(Screen *screen);
        virtual ~Area();

    public:
        void add(Widget *widget, bool manage=true);
        void remove(Widget *widget);
        void handleEvent(const SDL_Event &event);
        void run();
        void finishEventLoop();
        virtual void draw();
        void setTimer(Uint32 interval, TimerHandler *handler);
        void updateMouse();
        virtual bool destroyByArea() { return false; };
};


class ExitCommand: public Command
{
    private:
        Area &area;
    
    public:
        ExitCommand(Area &a): area(a) { }
        
        virtual void doAction() {
            area.finishEventLoop();
        };
};


class AnyKeyAccel: public Widget
{
    protected:
        Command *command;

    public:
        AnyKeyAccel(Screen *screen);                  // use exit command by default
        AnyKeyAccel(Screen *screen, Command *command);
        virtual ~AnyKeyAccel();

    public:
        virtual bool onKeyDown(SDLKey key, unsigned char ch);
        virtual bool onMouseButtonDown(int button, int x, int y);
};


class Window: public Widget
{
    protected:
        int left, top, width, height;
        SDL_Surface *background;
    
    public:
        Window(Screen *screen, int x, int y, int w, int h, const std::wstring &background, 
                bool frameWidth=4, bool raised=true);
        virtual ~Window();

    public:
        virtual void draw();
};


class Label: public Widget
{
    public:
        typedef enum {
            ALIGN_LEFT,
            ALIGN_CENTER,
            ALIGN_RIGHT
        } HorAlign;
        
        typedef enum {
            ALIGN_TOP,
            ALIGN_MIDDLE,
            ALIGN_BOTTOM
        } VerAlign;
    
    protected:
        Font *font;
        std::wstring text;
        int left, top, width, height;
        int red, green, blue;
        HorAlign hAlign;
        VerAlign vAlign;
        bool shadow;

    public:
        Label(Screen *screen, Font *font, int x, int y, int r, int g, int b, 
                std::wstring text, bool shadow=true);
        Label(Screen *screen, Font *font, int x, int y, int width, int height,
                HorAlign hAlign, VerAlign vAlign, int r, int g, int b, 
                const std::wstring &text);

    public:
        virtual void draw();
};


class Picture: public Widget
{
    protected:
        int left;
        int top;
        int width;
        int height;
        SDL_Surface *image;
        bool managed;
        
    public:
        Picture(Screen *screen, int x, int y, const std::wstring &name, bool transparent=true);
        Picture(Screen *screen, int x, int y, SDL_Surface *image);
        virtual ~Picture();

    public:
        virtual void draw();
        void moveX(const int newX);
        void getBounds(int &l, int &t, int &w, int &h);
        int getLeft() const { return left; };
        int getTop() const { return top; };
        int getWidth() const { return width; };
        int getHeight() const { return height; };
        
};

#endif
