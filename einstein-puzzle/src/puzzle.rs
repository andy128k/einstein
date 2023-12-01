use crate::puzzle_gen::generate_puzzle;
use crate::rules::{apply, Kind, Possibilities, PuzzleSize, Rule, SolvedPuzzle, Thing, Value};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Puzzle {
    pub solved_puzzle: SolvedPuzzle,
    pub rules: Vec<Rule>,
    pub possibilities: Possibilities,
}

pub enum PuzzleState {
    NotSolved,
    Solved,
    Failed,
}

impl Puzzle {
    pub fn new(size: PuzzleSize, rng: &mut impl Rng) -> Self {
        let (solved_puzzle, rules) = generate_puzzle(size, rng).expect("Puzzle is generated");

        let mut possibilities = Possibilities::new(solved_puzzle.size());
        for rule in &rules {
            if let Rule::Open(..) = *rule {
                possibilities = apply(&possibilities, rule);
            }
        }

        Self {
            solved_puzzle,
            rules,
            possibilities,
        }
    }

    pub fn size(&self) -> PuzzleSize {
        self.solved_puzzle.size()
    }

    pub fn is_valid(&self) -> bool {
        self.possibilities.is_valid(&self.solved_puzzle)
    }

    pub fn state(&self) -> PuzzleState {
        if !self.is_valid() {
            PuzzleState::Failed
        } else if self.possibilities.is_solved() {
            PuzzleState::Solved
        } else {
            PuzzleState::NotSolved
        }
    }

    pub fn exclude(&mut self, col: u8, row: Kind, value: Value) {
        let thing = Thing { row, value };
        if self.possibilities.is_possible(col, thing) {
            self.possibilities = self.possibilities.exclude(col, row, value);
        }
    }

    pub fn set(&mut self, col: u8, row: Kind, value: Value) {
        let thing = Thing { row, value };
        if self.possibilities.is_possible(col, thing) {
            self.possibilities = self.possibilities.set(col, row, value);
        }
    }

    pub fn restart(&mut self) {
        let mut possibilities = Possibilities::new(self.solved_puzzle.size());
        for rule in &self.rules {
            if let Rule::Open(..) = *rule {
                possibilities = apply(&possibilities, rule);
            }
        }
        self.possibilities = possibilities;
    }
}

impl std::default::Default for Puzzle {
    fn default() -> Self {
        Self::new(PuzzleSize::default(), &mut thread_rng())
    }
}
