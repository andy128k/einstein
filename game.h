#ifndef __GAME_H__
#define __GAME_H__


#include <iostream>
#include "verthints.h"
#include "horhints.h"
#include "puzzle.h"
#include "screen.h"
#include "rules.h"
#include "config.h"
#include "topscores.h"



class Watch;



class Game
{
    private:
        SolvedPuzzle *solvedPuzzle;
        RulesArr rules;
        Possibilities *possibilities;
        VertHints *verHints;
        HorHints *horHints;
        IconSet iconSet;
        Puzzle *puzzle;
        Watch *watch;
        bool hinted;
        SolvedPuzzle *savedSolvedPuzzle;
        RulesArr savedRules;
        Screen *screen;

    public:
        Game(Screen *screen);
        Game(Screen *screen, std::istream &stream);
        ~Game();

    public:
        SolvedPuzzle& getSolvedPuzzle() { return *solvedPuzzle; };
        RulesArr& getRules() { return rules; };
        Possibilities* getPossibilities() { return possibilities; };
        VertHints* getVerHints() { return verHints; };
        HorHints* getHorHints() { return horHints; };
        void save(std::ostream &stream);
        void run(Config* config, TopScores *top_scores);
        bool isHinted() { return hinted; };
        void setHinted() { hinted = true; };
        void restart();
        void newGame();

    private:
        void deleteRules();
        void pleaseWait();
        void genPuzzle();
        void resetVisuals();
};

#endif
