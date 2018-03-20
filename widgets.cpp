#include "widgets.h"
#include "utils.h"
#include "sound.h"


//////////////////////////////////////////////////////////////////
//
// Button
//
//////////////////////////////////////////////////////////////////


Button::Button(Screen *screen, int x, int y, const std::wstring &name, Command *cmd, 
        bool transparent)
    : Widget(screen)
{
    image = loadImage(name, transparent);
    highlighted = adjust_brightness(image, 1.5, transparent);

    left = x;
    top = y;
    width = image->w;
    height = image->h;

    mouseInside = false;
    command = cmd;
}


Button::Button(Screen *screen, int x, int y, int w, int h, Font *font, 
        int fR, int fG, int fB, int hR, int hG, int hB,
        const std::wstring &text, Command *cmd)
    : Widget(screen)
{
    left = x;
    top = y;
    width = w;
    height = h;

    SDL_Surface *s = SDL_CreateRGBSurface(SDL_SWSURFACE, w, h,
            24, 0x00FF0000, 0x0000FF00, 0x000000FF, 0/*0xFF000000*/);
    SDL_Rect src = { x, y, width, height };
    SDL_Rect dst = { 0, 0, width, height };
    SDL_BlitSurface(screen->getSurface(), &src, s, &dst);
    
    int tW, tH;
    font->getSize(text, tW, tH);
    font->draw(s, (width - tW) / 2, (height - tH) / 2, fR, fG, fB, true, text);
    image = SDL_DisplayFormat(s);
    SDL_BlitSurface(screen->getSurface(), &src, s, &dst);
    font->draw(s, (width - tW) / 2, (height - tH) / 2, hR, hG, hB, true, text);
    highlighted = SDL_DisplayFormat(s);
    SDL_FreeSurface(s);
    
    mouseInside = false;
    command = cmd;
}


Button::Button(Screen *screen, int x, int y, int w, int h, Font *font, 
        int r, int g, int b, const std::wstring &bg, 
        const std::wstring &text, bool bevel, Command *cmd)
    : Widget(screen)
{
    left = x;
    top = y;
    width = w;
    height = h;

    SDL_Surface *s = screen->getSurface();
    image = SDL_CreateRGBSurface(SDL_SWSURFACE, width, height, 
            s->format->BitsPerPixel, s->format->Rmask, s->format->Gmask,
            s->format->Bmask, s->format->Amask);

    SDL_Surface *tile = loadImage(bg, true);
    SDL_Rect src = { 0, 0, tile->w, tile->h };
    SDL_Rect dst = { 0, 0, tile->w, tile->h };
    for (int j = 0; j < height; j += tile->h)
        for (int i = 0; i < width; i += tile->w) {
            dst.x = i;
            dst.y = j;
            SDL_BlitSurface(tile, &src, image, &dst);
        }
    SDL_FreeSurface(tile);

    if (bevel) {
        SDL_LockSurface(image);
        drawBevel(image, 0, 0, width, height, false, 1);
        drawBevel(image, 1, 1, width - 2, height - 2, true, 1);
        SDL_UnlockSurface(image);
    }
    
    int tW, tH;
    font->getSize(text, tW, tH);
    font->draw(image, (width - tW) / 2, (height - tH) / 2, r, g, b, true, text);
    
    highlighted = adjust_brightness(image, 1.5, false);
    SDL_SetColorKey(image, SDL_SRCCOLORKEY, getCornerPixel(image));
    SDL_SetColorKey(highlighted, SDL_SRCCOLORKEY, getCornerPixel(highlighted));
    
    mouseInside = false;
    command = cmd;
}



Button::Button(Screen *screen, int x, int y, int w, int h, Font *font, 
        int r, int g, int b, const std::wstring &bg, 
        const std::wstring &text, Command *cmd)
    : Widget(screen)
{
    left = x;
    top = y;
    width = w;
    height = h;

    SDL_Surface *s = screen->getSurface();
    image = SDL_CreateRGBSurface(SDL_SWSURFACE, width, height, 
            s->format->BitsPerPixel, s->format->Rmask, s->format->Gmask,
            s->format->Bmask, s->format->Amask);

    SDL_Surface *tile = loadImage(bg);
    SDL_Rect src = { 0, 0, tile->w, tile->h };
    SDL_Rect dst = { 0, 0, tile->w, tile->h };
    for (int j = 0; j < height; j += tile->h)
        for (int i = 0; i < width; i += tile->w) {
            dst.x = i;
            dst.y = j;
            SDL_BlitSurface(tile, &src, image, &dst);
        }
    SDL_FreeSurface(tile);

    SDL_LockSurface(image);
    drawBevel(image, 0, 0, width, height, false, 1);
    drawBevel(image, 1, 1, width - 2, height - 2, true, 1);
    SDL_UnlockSurface(image);
    
    int tW, tH;
    font->getSize(text, tW, tH);
    font->draw(image, (width - tW) / 2, (height - tH) / 2, r, g, b, true, text);
    
    highlighted = adjust_brightness(image, 1.5, false);
    
    mouseInside = false;
    command = cmd;
}


Button::~Button()
{
    SDL_FreeSurface(image);
    SDL_FreeSurface(highlighted);
}


void Button::draw()
{
    if (mouseInside)
        screen->draw(left, top, highlighted);
    else
        screen->draw(left, top, image);
    screen->addRegionToUpdate(left, top, width, height);
}


void Button::getBounds(int &l, int &t, int &w, int &h)
{
    l = left;
    t = top;
    w = width;
    h = height;
}


bool Button::onMouseButtonDown(int button, int x, int y)
{
    if (isInRect(x, y, left, top, width, height)) {
        sound->play(L"click.wav");
        if (command)
            command->doAction();
        return true;
    } else
        return false;
}


bool Button::onMouseMove(int x, int y)
{
    bool in = isInRect(x, y, left, top, width, height);
    if (in != mouseInside) {
        mouseInside = in;
        draw();
    }
    return false;
}


//////////////////////////////////////////////////////////////////
//
// KeyAccel
//
//////////////////////////////////////////////////////////////////


KeyAccel::KeyAccel(Screen *screen, SDLKey sym, Command *cmd)
    : Widget(screen)
{
    command = cmd;
    key = sym;
}


bool KeyAccel::onKeyDown(SDLKey k, unsigned char ch)
{
    if (key == k) {
        if (command)
            command->doAction();
        return true;
    } else
        return false;
}


//////////////////////////////////////////////////////////////////
//
// Area
//
//////////////////////////////////////////////////////////////////


Area::Area(Screen *screen)
    : Widget(screen)
{
    timer = NULL;
}

Area::~Area()
{
    for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++) {
        Widget *w = *i;
        if (w && w->destroyByArea() && (! notManagedWidgets.count(w)))
            delete w;
    }
}

void Area::add(Widget *widget, bool managed)
{
    widgets.push_back(widget);
    if (! managed)
        notManagedWidgets.insert(widget);
    widget->setParent(this);
}

void Area::remove(Widget *widget)
{
    widgets.remove(widget);
    notManagedWidgets.insert(widget);
}

void Area::handleEvent(const SDL_Event &event)
{
    switch (event.type) {
        case SDL_MOUSEBUTTONDOWN:
            for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
                if ((*i)->onMouseButtonDown(event.button.button, 
                            event.button.x, event.button.y))
                    return;
            break;
        
        case SDL_MOUSEBUTTONUP:
            for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
                if ((*i)->onMouseButtonUp(event.button.button, 
                            event.button.x, event.button.y))
                    return;
            break;
        
        case SDL_MOUSEMOTION:
            for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
                if ((*i)->onMouseMove(event.motion.x, event.motion.y))
                    return;
            break;
        
        case SDL_VIDEOEXPOSE:
            for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
                (*i)->draw();
            break;
        
        case SDL_KEYDOWN:
            for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
                if ((*i)->onKeyDown(event.key.keysym.sym, 
                            (unsigned char)event.key.keysym.unicode))
                    return;
            break;
        
        case SDL_QUIT:
            exit(0);
    }
}

void Area::run()
{
    terminate = false;
    SDL_Event event;
    
    Uint32 lastTimer = 0;
    draw();
    
    bool runTimer = timer ? true : false;
    bool dispetchEvent;
    while (! terminate) {
        dispetchEvent = true;
        if (! timer) {
            SDL_WaitEvent(&event);
        } else {
            Uint32 now = SDL_GetTicks();
            if (now - lastTimer > time) {
                lastTimer = now;
                runTimer = true;
            }
            if (! SDL_PollEvent(&event)) {
                if (! runTimer) {
                    SDL_Delay(20);
                    continue;
                } else
                    dispetchEvent = false;
            }
        }
        if (runTimer) {
            if (timer)
                timer->onTimer();
            runTimer = false;
        }
        if (dispetchEvent)
            handleEvent(event);
        if (! terminate) {
            screen->flush();
        }
    }
}

void Area::finishEventLoop()
{
    terminate = true;
}


void Area::draw()
{
    for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
        (*i)->draw();
}


void Area::setTimer(Uint32 interval, TimerHandler *t)
{
    time = interval;
    timer = t;
}


void Area::updateMouse()
{
    int x, y;
    SDL_GetMouseState(&x, &y);
    
    for (WidgetsList::iterator i = widgets.begin(); i != widgets.end(); i++)
        if ((*i)->onMouseMove(x, y))
                    return;
}



//////////////////////////////////////////////////////////////////
//
// AnyKeyAccel
//
//////////////////////////////////////////////////////////////////


AnyKeyAccel::AnyKeyAccel(Screen *screen)
    : Widget(screen)
{
    command = NULL;
}

AnyKeyAccel::AnyKeyAccel(Screen *screen, Command *cmd)
    : Widget(screen)
{
    command = cmd;
}

AnyKeyAccel::~AnyKeyAccel()
{
}

bool AnyKeyAccel::onKeyDown(SDLKey key, unsigned char ch)
{
    if (((key >= SDLK_NUMLOCK) && (key <= SDLK_COMPOSE)) || 
            (key == SDLK_TAB) || (key == SDLK_UNKNOWN))
        return false;

    if (command)
        command->doAction();
    else
        area->finishEventLoop();
    return true;
}

bool AnyKeyAccel::onMouseButtonDown(int button, int x, int y)
{
    if (command)
        command->doAction();
    else
        area->finishEventLoop();
    return true;
}



//////////////////////////////////////////////////////////////////
//
// Window
//
//////////////////////////////////////////////////////////////////



Window::Window(Screen *screen, int x, int y, int w, int h, const std::wstring &bg, 
                bool frameWidth, bool raised)
    : Widget(screen)
{
    left = x;
    top = y;
    width = w;
    height = h;
    
    SDL_Surface *s = screen->getSurface();
    SDL_Surface *win = SDL_CreateRGBSurface(SDL_SWSURFACE, width, height, 
            s->format->BitsPerPixel, s->format->Rmask, s->format->Gmask,
            s->format->Bmask, s->format->Amask);

    SDL_Surface *tile = loadImage(bg);
    SDL_Rect src = { 0, 0, tile->w, tile->h };
    SDL_Rect dst = { 0, 0, tile->w, tile->h };
    for (int j = 0; j < height; j += tile->h)
        for (int i = 0; i < width; i += tile->w) {
            dst.x = i;
            dst.y = j;
            SDL_BlitSurface(tile, &src, win, &dst);
        }
    SDL_FreeSurface(tile);

    SDL_LockSurface(win);
    double k = 2.6;
    double f = 0.1;
    for (int i = 0; i < frameWidth; i++) {
        double ltK, rbK;
        if (raised) {
            ltK = k;  rbK = f;
        } else {
            ltK = f;  rbK = k;
        }
        for (int j = i; j < height - i - 1; j++)
            adjust_brightness_pixel(win, i, j, ltK);
        for (int j = i; j < width - i; j++)
            adjust_brightness_pixel(win, j, i, ltK);
        for (int j = i+1; j < height - i; j++)
            adjust_brightness_pixel(win, width - i - 1, j, rbK);
        for (int j = i; j < width - i - 1; j++)
            adjust_brightness_pixel(win, j, height - i - 1, rbK);
        k -= 0.2;
        f += 0.1;
    }
    SDL_UnlockSurface(win);
    
    background = SDL_DisplayFormat(win);
    SDL_FreeSurface(win);
}


Window::~Window()
{
    SDL_FreeSurface(background);
}


void Window::draw()
{
    screen->draw(left, top, background);
    screen->addRegionToUpdate(left, top, width, height);
}



//////////////////////////////////////////////////////////////////
//
// Label
//
//////////////////////////////////////////////////////////////////



Label::Label(Screen *screen, Font *f, int x, int y, int r, int g, int b, std::wstring s,
        bool sh)
    : Widget(screen), text(s)
{
    font = f;
    left = x;
    top = y;
    red = r;
    green = g;
    blue = b;
    hAlign = ALIGN_LEFT;
    vAlign = ALIGN_TOP;
    shadow = sh;
}


Label::Label(Screen *screen, Font *f, int x, int y, int w, int h, HorAlign hA, VerAlign vA, 
        int r, int g, int b, const std::wstring &s)
    : Widget(screen), text(s)
{
    font = f;
    left = x;
    top = y;
    red = r;
    green = g;
    blue = b;
    hAlign = hA;
    vAlign = vA;
    width = w;
    height = h;
    shadow = true;
}


void Label::draw()
{
    int w, h, x, y;
    font->getSize(text, w, h);

    switch (hAlign) {
        case ALIGN_RIGHT: x = left + width - w; break;
        case ALIGN_CENTER: x = left + (width - w) / 2; break;
        default: x = left;
    }
    
    switch (vAlign) {
        case ALIGN_BOTTOM: y = top + height - h; break;
        case ALIGN_MIDDLE: y = top + (height - h) / 2; break;
        default: y = top;
    }
    
    font->draw(screen->getSurface(), x, y, red,green,blue, shadow, text);
    screen->addRegionToUpdate(x, y, w, h);
}


//////////////////////////////////////////////////////////////////////////////
//
// Picture
//
//////////////////////////////////////////////////////////////////////////////

Picture::Picture(Screen *screen, int x, int y, const std::wstring &name, bool transparent)
    : Widget(screen)
{
    image = loadImage(name, transparent);
    left = x;
    top = y;
    width = image->w;
    height = image->h;
    managed = true;
}

Picture::Picture(Screen *screen, int x, int y, SDL_Surface *img)
    : Widget(screen)
{
    image = img;
    left = x;
    top = y;
    width = image->w;
    height = image->h;
    managed = false;
}

Picture::~Picture() 
{ 
    if (managed)
        SDL_FreeSurface(image); 
}

void Picture::draw()
{
    screen->draw(left, top, image);
    screen->addRegionToUpdate(left, top, width, height);
}

void Picture::moveX(const int newX) 
{ 
    left = newX; 
}

void Picture::getBounds(int &l, int &t, int &w, int &h)
{
    l = left;
    t = top;
    w = width;
    h = height;
}
