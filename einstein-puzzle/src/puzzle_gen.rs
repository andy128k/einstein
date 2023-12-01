use crate::rules::*;
use crate::util::{converge::converge_result, retry::retry};
use rand::Rng;

#[derive(Debug)]
pub struct InvalidPuzzle(Rule);

impl std::error::Error for InvalidPuzzle {}

impl std::fmt::Display for InvalidPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid possibilities after rule {}", self.0)
    }
}

fn solve(puzzle: &SolvedPuzzle, rules: &[Rule]) -> Result<Possibilities, InvalidPuzzle> {
    let possibilities = converge_result(Possibilities::new(puzzle.size()), |mut possibilities| {
        for rule in rules {
            possibilities = apply(&possibilities, rule);
            if !possibilities.is_valid(puzzle) {
                return Err(InvalidPuzzle(*rule));
            }
        }
        Ok(possibilities)
    })?;
    Ok(possibilities)
}

fn can_solve(puzzle: &SolvedPuzzle, rules: &[Rule]) -> Result<bool, InvalidPuzzle> {
    let possibilities = solve(puzzle, rules)?;
    Ok(possibilities.is_solved())
}

fn remove_rules(puzzle: &SolvedPuzzle, rules: &[Rule]) -> Result<Vec<Rule>, InvalidPuzzle> {
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

fn generate_rules(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Result<Vec<Rule>, InvalidPuzzle> {
    let mut rules: Vec<Rule> = Vec::new();
    while !can_solve(puzzle, &rules)? {
        let rule = retry(|| generate_rule(rng, puzzle), |rule| !rules.contains(rule));
        rules.push(rule);
    }
    Ok(rules)
}

pub fn generate_puzzle(
    size: PuzzleSize,
    rng: &mut impl Rng,
) -> Result<(SolvedPuzzle, Vec<Rule>), InvalidPuzzle> {
    let puzzle = SolvedPuzzle::random(size, rng);
    let rules = generate_rules(rng, &puzzle)?;
    let reduced_rules = remove_rules(&puzzle, &rules)?;
    Ok((puzzle, reduced_rules))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_eq_generate_puzzle() {
        let size = PuzzleSize {
            kinds: 6,
            values: 6,
        };
        let mut rng = thread_rng();
        let (_puzzle, rules) = generate_puzzle(size, &mut rng).unwrap();
        assert!(rules.len() > 0);
    }
}
