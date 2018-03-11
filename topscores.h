#ifndef __TOPSCORES_H__
#define __TOPSCORES_H__


#include <list>
#include <string>
#include "widgets.h"


extern "C" {
    struct TopScores;

    int ein_topscores_is_deserving(TopScores*, int);
    int ein_topscores_add(TopScores*, const char*, int);
}

void showScoresWindow(Area *area, TopScores *scores, int highlightPos=-1);

std::wstring enterNameDialog(Area *area, std::wstring &name);

#endif
