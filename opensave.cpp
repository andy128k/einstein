#include <time.h>
#include <fstream>
#include "exceptions.h"
#include "utils.h"
#include "widgets.h"
#include "opensave.h"
#include "unicode.h"
#include "convert.h"
#include "messages.h"



#define MAX_SLOTS 10


class SavedGame
{
    private:
        std::wstring fileName;
        bool exists;
        std::wstring name;

    public:
        SavedGame(const std::wstring &fileName);
        SavedGame(const SavedGame &s): fileName(s.fileName), name(s.name) {
            exists = s.exists;
        };

    public:
        const std::wstring& getFileName() { return fileName; };
        std::wstring getName() { return exists ? name : msg(L"empty"); };
        bool isExists() { return exists; };
};


SavedGame::SavedGame(const std::wstring &s): fileName(s)
{
    exists = false;
    
    try {
        std::ifstream stream(toMbcs(fileName).c_str(), std::ifstream::in | 
                std::ifstream::binary);
        if (stream.fail())
            throw Exception(L"Can't open file");
        name = readString(stream);
        stream.close();
        exists = true;
    } catch (...) { }
}



class OkCommand: public Command
{
    private:
        Area &area;
        bool *ok;
    
    public:
        OkCommand(Area &a, bool *o): area(a) { ok = o; };
        
        virtual void doAction() {
            *ok = true;
            area.finishEventLoop();
        };
};


static std::wstring getSavesPath()
{
    std::wstring path(fromMbcs(getenv("HOME")) + std::wstring(L"/.einstein/save"));
    ensureDirExists(path);
    return path;
}


typedef std::list<SavedGame> SavesList;


static void showListWindow(SavesList &list, Command **commands,
        const std::wstring &title, Area &area, Font *font)
{
    Screen *screen = area.screen;

    Font titleFont(L"nova.ttf", 26);

    area.add(new Window(screen, 250, 90, 300, 420, L"blue.bmp"));
    area.add(new Label(screen, &titleFont, 250, 95, 300, 40, Label::ALIGN_CENTER,
                Label::ALIGN_MIDDLE, 255,255,0, title));
    ExitCommand exitCmd(area);
    area.add(new Button(screen, 360, 470, 80, 25, font, 255,255,0, L"blue.bmp", 
                msg(L"close"), &exitCmd));
    area.add(new KeyAccel(screen, SDLK_ESCAPE, &exitCmd)); 

    int pos = 150;
    int no = 0;
    for (SavesList::iterator i = list.begin(); i != list.end(); i++) {
        SavedGame &game = *i;
        area.add(new Button(screen, 260, pos, 280, 25, font, 255,255,255, L"blue.bmp", 
                    game.getName(), commands[no++]));
        pos += 30;
    }
    
    area.run();
}


class LoadCommand: public Command
{
    private:
        SavedGame &savedGame;
        Area *parentArea;
        bool *saved;
        Font *font;
        std::wstring defaultName;
        Game **game;

    public:
        LoadCommand(SavedGame &sg, Font *f, Area *area, Game **g): 
            savedGame(sg)
        {
            parentArea = area;
            font = f;
            game = g;
        };
        
    public:
        virtual void doAction() {
            try {
                std::ifstream stream(toMbcs(savedGame.getFileName()).c_str(), 
                        std::ifstream::in | std::ifstream::binary);
                if (stream.fail())
                    throw Exception(L"Error opening save file");
                readString(stream);
                Game *g = new Game(parentArea->screen, stream);
                if (stream.fail())
                    throw Exception(L"Error loading game");
                stream.close();
                *game = g;
            } catch (...) { 
                showMessageWindow(parentArea->screen, parentArea, L"redpattern.bmp", 300, 80, font,
                        255,255,255, L"Error loadng game");
            }
            parentArea->finishEventLoop();
        };
};


Game* loadGame(Area *parentArea)
{
    Screen *screen = parentArea->screen;

    std::wstring path = getSavesPath();
    
    Area area(screen);
    area.add(parentArea, false);
    Font font(L"laudcn2.ttf", 14);
    
    Game *newGame = NULL;
    
    SavesList list;
    Command **commands = new Command*[MAX_SLOTS];
    for (int i = 0; i < MAX_SLOTS; i++) {
        SavedGame sg(path + L"/" + toString(i) + L".sav");
        list.push_back(sg);
        if (sg.isExists())
            commands[i] = new LoadCommand(*(--(list.end())), &font, &area, 
                    &newGame);
        else
            commands[i] = NULL;
    }
    
    showListWindow(list, commands, msg(L"loadGame"), area, &font);

    for (int i = 0; i < MAX_SLOTS; i++)
        delete commands[i];
    delete[] commands;
   
    return newGame;
}

