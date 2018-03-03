use rules::*;
use error::*;
use converge::{converge, converge_result};

fn can_solve(puzzle: &SolvedPuzzle, rules: &[Rule]) -> Result<bool> {
    let pos = converge_result(Possibilities::new(), |mut pos| {
        for rule in rules {
            pos = apply(&pos, rule);
            if !pos.is_valid(puzzle) {
                return Err(format_err!("Invalid possibilities after rule {}", display_rule(rule)));
            }
        }
        Ok(pos)
    })?;
    Ok(pos.is_solved())
}

fn remove_rules(puzzle: &SolvedPuzzle, rules: &[Rule]) -> Result<Vec<Rule>> {
    converge_result(rules.to_vec(), |rules| {
        for index in 0..rules.len() {
            let mut excluded_rules = rules.clone();
            excluded_rules.remove(index);
            if can_solve(puzzle, &excluded_rules)? {
                return Ok(excluded_rules);
            }
        }
        Ok(rules)
    })
}

fn generate_rules(puzzle: &SolvedPuzzle) -> Result<Vec<Rule>> {
    let mut rules: Vec<Rule> = Vec::new();
    while !can_solve(puzzle, &rules)? {
        loop {
            let rule = generate_rule(puzzle);
            if rules.iter().find(|r| **r == rule).is_none() {
                rules.push(rule);
                break;
            }
        }
    }
    Ok(rules)
}

/*
void genPuzzle(SolvedPuzzle &puzzle, Rules &rules)
{
    for (int i = 0; i < PUZZLE_SIZE; i++) {
        for (int j = 0; j < PUZZLE_SIZE; j++) 
            puzzle[i][j] = j + 1;
        shuffle(puzzle[i]);
    }

    genRules(puzzle, rules);
    removeRules(puzzle, rules);
}


void openInitial(Possibilities &possib, Rules &rules)
{
    for (Rules::iterator i = rules.begin(); i != rules.end(); i++) {
        Rule *r = *i;
        if (r->applyOnStart())
            r->apply(possib);
    }
}
*/
