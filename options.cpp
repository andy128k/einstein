#include "options.h"
#include "messages.h"
#include "sound.h"


class OptionsChangedCommand: public Command
{
    private:
        bool &fullscreen;
        float &volume;
        Area *area;
        Config *config;
    
    public:
        OptionsChangedCommand(Area *a, Config *config, bool &fs, float &v): 
            fullscreen(fs), volume(v) {
            area = a;
            this->config = config;
        };

        virtual void doAction() {
            Screen *screen = area->screen;

            bool oldFullscreen = ein_config_get_fullscreen(config) != 0;
            float oldVolume = ((float)ein_config_get_volume(config)) / 100.0f;

            if (fullscreen != oldFullscreen) {
                ein_config_set_fullscreen(config, fullscreen);
                screen->setMode(VideoMode(800, 600, 24, fullscreen));
            }

            if (volume != oldVolume) {
                ein_config_set_volume(config, (int)(volume * 100.0f));
                sound->setVolume(volume);
            }
            area->finishEventLoop();
        };
};


#define LABEL(y, s) \
    area.add(new Label(screen, &font, 300, y, 300, 20, Label::ALIGN_LEFT, \
                Label::ALIGN_MIDDLE, 255,255,255, msg(s)));
#define CHECKBOX(y, var) \
    area.add(new Checkbox(screen, 265, y, 20, 20, &font, 255,255,255, L"blue.bmp", \
                var));
#define OPTION(y, s, var) LABEL(y, s) CHECKBOX(y, var)

void showOptionsWindow(Area *parentArea, Config *config)
{
    Font titleFont(L"nova.ttf", 26);
    Font font(L"laudcn2.ttf", 14);

    bool fullscreen = ein_config_get_fullscreen(config) != 0;
    float volume = ((float)ein_config_get_volume(config)) / 100.0f;
    
    Screen *screen = parentArea->screen;

    Area area(screen);

    area.add(parentArea);
    area.add(new Window(screen, 250, 170, 300, 260, L"blue.bmp"));
    area.add(new Label(screen, &titleFont, 250, 175, 300, 40, Label::ALIGN_CENTER,
                Label::ALIGN_MIDDLE, 255,255,0, msg(L"options")));
    OPTION(260, L"fullscreen", fullscreen);
    
    area.add(new Label(screen, &font, 265, 330, 300, 20, Label::ALIGN_LEFT,
                Label::ALIGN_MIDDLE, 255,255,255, msg(L"volume")));
    area.add(new Slider(screen, 360, 332, 160, 16, volume));
    
    ExitCommand exitCmd(area);
    OptionsChangedCommand okCmd(&area, config, fullscreen, volume);
    area.add(new Button(screen, 315, 390, 85, 25, &font, 255,255,0, L"blue.bmp", 
                msg(L"ok"), &okCmd));
    area.add(new Button(screen, 405, 390, 85, 25, &font, 255,255,0, L"blue.bmp", 
                msg(L"cancel"), &exitCmd));
    area.add(new KeyAccel(screen, SDLK_ESCAPE, &exitCmd));
    area.add(new KeyAccel(screen, SDLK_RETURN, &okCmd));
    area.run();
}
