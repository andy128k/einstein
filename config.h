#ifndef __CONFIG_H__
#define __CONFIG_H__

extern "C" {
    struct Config;

    const char* ein_config_get_last_name(Config*);
    void ein_config_set_last_name(Config*, const char*);

    int ein_config_get_fullscreen(Config*);
    void ein_config_set_fullscreen(Config*, int);

    int ein_config_get_volume(Config*);
    void ein_config_set_volume(Config*, int);
}

#endif
