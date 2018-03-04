use rules::*;
use error::*;
use converge::converge_result;

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

pub fn generate_puzzle() -> Result<(SolvedPuzzle, Vec<Rule>)> {
    loop {
        let puzzle = SolvedPuzzle::random();
        let rules = generate_rules(&puzzle)?;
        let reduced_rules = remove_rules(&puzzle, &rules)?;

        let mut horizontal = 0;
        let mut vertical = 0;
        for rule in &reduced_rules {
            match *rule {
                Rule::Near(..) |
                Rule::Between(..) |
                Rule::Direction(..) => horizontal += 1,
                Rule::Under(..) => vertical += 1,
                Rule::Open(..) => {},
            }
        }

        if horizontal <= 24 && vertical <= 15 {
            return Ok((puzzle, reduced_rules));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_generate_puzzle() {
        let (_puzzle, rules) = generate_puzzle().unwrap();
        assert!(rules.len() > 0);
    }
}
