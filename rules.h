#ifndef __PUZZLE_RULES_H__
#define __PUZZLE_RULES_H__

#include <memory.h>
#include <vector>

struct SolvedPuzzle;
struct Rule;
struct Possibilities;

typedef std::vector<Rule*> RulesArr;

extern "C" {
    void ein_generate_puzzle(SolvedPuzzle**, Rule**, size_t*);
    SolvedPuzzle* ein_solved_puzzle_clone(SolvedPuzzle*);
    void ein_solved_puzzle_free(SolvedPuzzle*);

    int ein_rule_is_vertical(Rule*);
    int ein_rule_is_horizontal(Rule*);
    void ein_rule_draw(Rule*, SDL_Surface*, int, int, int);
    void ein_rule_free(Rule*);

    Possibilities* ein_possibilities_new();
    Possibilities* ein_possibilities_open_initials(Possibilities*, Rule**, size_t);

    int ein_possibilities_is_possible(Possibilities*, int, int, int);
    Possibilities* ein_possibilities_set(Possibilities*, int, int, int);
    Possibilities* ein_possibilities_exclude(Possibilities*, int, int, int);
    int ein_possibilities_is_defined(Possibilities*, int, int);
    int ein_possibilities_get_defined(Possibilities*, int, int);

    int ein_possibilities_is_valid(Possibilities*, SolvedPuzzle*);
    int ein_possibilities_is_solved(Possibilities*);

    void ein_possibilities_free(Possibilities*);

    void ein_draw_thing(int t, int v, SDL_Surface*, int x, int y, int h);
    void ein_draw_small_thing(int t, int v, SDL_Surface*, int x, int y, int h);
}

#endif
