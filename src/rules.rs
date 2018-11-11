use std::fmt;
use itertools::all;
use rand::Rng;
use converge::converge;

fn only<T>(values: &[T]) -> Option<&T> {
    if values.len() == 1 {
        values.first()
    } else {
        None
    }
}


pub const PUZZLE_SIZE: usize = 6;

pub type Value = u8;

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Thing { pub row: u8, pub value: Value }

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.row {
            0 => write!(f, "{}", self.value),
            1 => write!(f, "{}", ["A", "B", "C", "D", "E", "F"][self.value as usize]),
            2 => write!(f, "{}", ["Ⅰ", "Ⅱ", "Ⅲ", "Ⅳ", "Ⅴ", "Ⅵ"][self.value as usize]),
            3 => write!(f, "{}", ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"][self.value as usize]),
            4 => write!(f, "{}", ["α", "β", "γ", "δ", "ε", "ζ"][self.value as usize]),
            5 => write!(f, "{}", ["+", "-", "÷", "*", "=", "√"][self.value as usize]),
            _ => unreachable!()
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SolvedPuzzle([[Value; PUZZLE_SIZE]; PUZZLE_SIZE]);

impl SolvedPuzzle {
    pub fn random(rng: &mut impl Rng) -> Self {
        let mut values = [[0u8; PUZZLE_SIZE]; PUZZLE_SIZE];
        for r in 0..PUZZLE_SIZE {
            for c in 0..PUZZLE_SIZE {
                values[r][c] = c as u8;
            }
            rng.shuffle(&mut values[r]);
        }
        SolvedPuzzle(values)
    }

    pub fn get(&self, row: u8, col: u8) -> Thing {
        Thing { row: row, value: self.0[row as usize][col as usize] }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ValueSet([bool; PUZZLE_SIZE]);

impl ValueSet {
    pub fn empty() -> Self {
        ValueSet([false; PUZZLE_SIZE])
    }

    pub fn full() -> Self {
        ValueSet([true; PUZZLE_SIZE])
    }

    pub fn single(value: Value) -> Self {
        let mut set = Self::empty();
        set.add(value);
        set
    }

    pub fn size(&self) -> usize {
        let mut result = 0;
        for v in self.0.iter() {
            if *v {
                result += 1;
            }
        }
        result
    }

    pub fn get_single(&self) -> Option<Value> {
        let mut result = None;
        for (value, present) in self.0.iter().enumerate() {
            if *present {
                if result.is_some() {
                    return None;
                }
                result = Some(value as Value);
            }
        }
        result
    }

    pub fn contains(&self, value: Value) -> bool {
        self.0[value as usize]
    }

    pub fn add(&mut self, value: Value) {
        self.0[value as usize] = true;
    }

    pub fn remove(&mut self, value: Value) -> bool {
        let present = self.0[value as usize];
        self.0[value as usize] = false;
        present
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PossibilitiesRow([ValueSet; PUZZLE_SIZE]);

impl PossibilitiesRow {
    pub fn new() -> Self {
        PossibilitiesRow([ValueSet::full(); PUZZLE_SIZE])
    }

    pub fn exclude(&self, col: u8, element: Value) -> Self {
        let mut new_row = *self;
        new_row.0[col as usize].remove(element);
        new_row.check_singles()
    }

    pub fn set(&self, col: u8, element: Value) -> Self {
        let mut new_row = *self;
        new_row.0[col as usize] = ValueSet::single(element);
        new_row.check_singles()
    }

    fn value_in_columns(&self, value: Value) -> Vec<u8> {
        (0..PUZZLE_SIZE)
            .filter(|col| self.0[*col].contains(value))
            .map(|col| col as u8)
            .collect()
    }

    fn check_single_value_in_a_cell(&self) -> Self {
        let mut new_row = *self;
        for col in 0..PUZZLE_SIZE {
            if let Some(value) = self.0[col].get_single() {
                // there is only one element in cell but it used somewhere else
                for i in 0..PUZZLE_SIZE {
                    if i != col {
                        new_row.0[i as usize].remove(value);
                    }
                }
            }
        }
        new_row
    }

    fn check_single_value_in_a_row(&self) -> Self {
        let mut new_row = *self;
        // check for single element without exclusive cell
        for el in 0..PUZZLE_SIZE {
            if let Some(col) = only(&new_row.value_in_columns(el as Value)) {
                new_row.0[*col as usize] = ValueSet::single(el as Value);
            }
        }
        new_row
    }

    fn check_singles(&self) -> Self {
        converge(*self, |p| p.check_single_value_in_a_cell().check_single_value_in_a_row())
    }

    pub fn is_solved(&self) -> bool {
        all(&self.0, |s| s.size() == 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Possibilities([PossibilitiesRow; PUZZLE_SIZE]);

impl Possibilities {
    pub fn new() -> Self {
        Possibilities([PossibilitiesRow::new(); PUZZLE_SIZE])
    }

    pub fn exclude(&self, col: u8, row: u8, element: Value) -> Self {
        let mut new = self.clone();
        new.0[row as usize] = new.0[row as usize].exclude(col, element);
        new
    }

    pub fn set(&self, col: u8, row: u8, element: Value) -> Self {
        let mut new = self.clone();
        new.0[row as usize] = new.0[row as usize].set(col, element);
        new
    }

    pub fn is_possible(&self, col: u8, thing: Thing) -> bool {
        self.0[thing.row as usize].0[col as usize].contains(thing.value)
    }

    pub fn is_defined(&self, col: u8, row: u8) -> bool {
        self.0[row as usize].0[col as usize].size() == 1
    }

    pub fn get_defined(&self, col: u8, row: u8) -> Option<Value> {
        self.0[row as usize].0[col as usize].get_single()
    }

    pub fn is_solved(&self) -> bool {
        all(&self.0, |s| s.is_solved())
    }

    pub fn is_valid(&self, puzzle: &SolvedPuzzle) -> bool {
        for row in 0..PUZZLE_SIZE {
            for col in 0..PUZZLE_SIZE {
                if !self.0[row as usize].0[col as usize].contains(puzzle.0[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Rule {
    Near(Thing, Thing),
    Direction(Thing, Thing),
    Open(u8, Thing), // column
    Under(Thing, Thing),
    Between(Thing, Thing, Thing)
}

fn generate_near_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let row1: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let row2: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let first_col: u8 = rng.gen_range(0, PUZZLE_SIZE as u8 - 1);

    let thing1 = puzzle.get(row1, first_col);
    let thing2 = puzzle.get(row2, first_col + 1);

    if rng.gen() {
        Rule::Near(thing1, thing2)
    } else {
        Rule::Near(thing2, thing1)
    }
}

fn generate_direction_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let row1: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let row2: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let col1: u8 = rng.gen_range(0, PUZZLE_SIZE as u8 - 1);
    let col2: u8 = rng.gen_range(col1 + 1, PUZZLE_SIZE as u8);

    let thing1 = puzzle.get(row1, col1);
    let thing2 = puzzle.get(row2, col2);
    Rule::Direction(thing1, thing2)
}

fn generate_open_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let row: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let col: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);

    let thing = puzzle.get(row, col);
    Rule::Open(col, thing)
}

fn generate_under_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let col: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let row1: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);

    let thing1 = puzzle.get(row1, col);

    loop {
        let row2: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
        if row1 != row2 {
            let thing2 = puzzle.get(row2, col);
            return Rule::Under(thing1, thing2);
        }
    }
}

fn generate_between_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let row1: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let row2: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let row3: u8 = rng.gen_range(0, PUZZLE_SIZE as u8);
    let first_col: u8 = rng.gen_range(0, PUZZLE_SIZE as u8 - 2);

    let thing1 = puzzle.get(row1, first_col);
    let thing2 = puzzle.get(row2, first_col + 1);
    let thing3 = puzzle.get(row3, first_col + 2);

    if rng.gen() {
        Rule::Between(thing1, thing2, thing3)
    } else {
        Rule::Between(thing3, thing2, thing1)
    }
}

pub fn generate_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    match rng.gen_range(0, 14) {
        0 | 1 | 2 | 3 => generate_near_rule(rng, puzzle),
        4 => generate_open_rule(rng, puzzle),
        5 | 6 => generate_under_rule(rng, puzzle),
        7 | 8 | 9 | 10 => generate_direction_rule(rng, puzzle),
        11 | 12 | 13 => generate_between_rule(rng, puzzle),
        _ => unreachable!()
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Near(thing1, thing2) => write!(f, "{} is near to {}", thing1, thing2),
            Rule::Direction(thing1, thing2) => write!(f, "{} is from the left of {}", thing1, thing2),
            Rule::Open(col, thing) => write!(f, "{} is at column {}", thing, col + 1),
            Rule::Under(thing1, thing2) => write!(f, "{} is the same column as {}", thing1, thing2),
            Rule::Between(thing1, thing2, thing3) => write!(f, "{} is between {} and {}", thing2, thing1, thing3)
        }
    }
}

pub fn apply(pos: &Possibilities, rule: &Rule) -> Possibilities {
    match *rule {
        Rule::Near(thing1, thing2) => {
            fn is_applicable_to_col(pos: &Possibilities, col: u8, thing: Thing, neighbour: Thing) -> bool {
                let has_left = if col == 0 {
                    false
                } else {
                    pos.is_possible(col - 1, neighbour)
                };

                let has_right = if col + 1 == PUZZLE_SIZE as u8 {
                    false
                } else {
                    pos.is_possible(col + 1, neighbour)
                };
                
                !has_right && !has_left && pos.is_possible(col, thing)
            }
            converge(*pos, |mut pos| {
                for col in 0..PUZZLE_SIZE {
                    if is_applicable_to_col(&pos, col as u8, thing1, thing2) {
                        pos = pos.exclude(col as u8, thing1.row, thing1.value);
                    }
                    if is_applicable_to_col(&pos, col as u8, thing2, thing1) {
                        pos = pos.exclude(col as u8, thing2.row, thing2.value);
                    }
                }
                pos
            })
        },
        Rule::Direction(thing1, thing2) => {
            let mut new_pos = *pos;
            for col in 0..PUZZLE_SIZE {
                if new_pos.is_possible(col as u8, thing2) {
                    new_pos = new_pos.exclude(col as u8, thing2.row, thing2.value);
                }
                if new_pos.is_possible(col as u8, thing1) {
                    break;
                }
            }
            for col in (0..PUZZLE_SIZE).rev() {
                if new_pos.is_possible(col as u8, thing1) {
                    new_pos = new_pos.exclude(col as u8, thing1.row, thing1.value);
                }
                if new_pos.is_possible(col as u8, thing2) {
                    break;
                }
            }
            new_pos
        },
        Rule::Open(col, thing) => {
            pos.set(col, thing.row, thing.value)
        },
        Rule::Under(thing1, thing2) => {
            let mut new_pos = *pos;
            for col in 0..PUZZLE_SIZE {
                if !new_pos.is_possible(col as u8, thing1) {
                    new_pos = new_pos.exclude(col as u8, thing2.row, thing2.value);
                }
                if !new_pos.is_possible(col as u8, thing2) {
                    new_pos = new_pos.exclude(col as u8, thing1.row, thing1.value);
                }
            }
            new_pos
        },
        Rule::Between(thing1, thing2, thing3) => {
            fn check_middle_thing(pos: &Possibilities, col: u8, thing1: Thing, thing2: Thing, thing3: Thing) -> bool {
                col > 0 && col < PUZZLE_SIZE as u8 - 1 &&
                pos.is_possible(col as u8, thing2) && (
                    (pos.is_possible(col as u8 - 1, thing1) && pos.is_possible(col as u8 + 1, thing3)) ||
                    (pos.is_possible(col as u8 - 1, thing3) && pos.is_possible(col as u8 + 1, thing1))
                )
            }

            fn check_side_thing(pos: &Possibilities, col: u8, thing1: Thing, thing2: Thing, thing3: Thing) -> bool {
                if pos.is_possible(col as u8, thing3) {
                    let left_possible = if col < 2 {
                        false
                    } else {
                        pos.is_possible(col as u8 - 1, thing2) && pos.is_possible(col as u8 - 2, thing1)
                    };
                    let right_possible = if col >= PUZZLE_SIZE as u8 - 2 {
                        false
                    } else {
                        pos.is_possible(col as u8 + 1, thing2) && pos.is_possible(col as u8 + 2, thing1)
                    };
                    left_possible || right_possible
                } else {
                    false
                }
            }

            converge(*pos, |mut pos| {
                for col in 0..PUZZLE_SIZE {
                    if !check_middle_thing(&pos, col as u8, thing1, thing2, thing3) {
                        pos = pos.exclude(col as u8, thing2.row, thing2.value);
                    }
                    if !check_side_thing(&pos, col as u8, thing1, thing2, thing3) {
                        pos = pos.exclude(col as u8, thing3.row, thing3.value);
                    }
                    if !check_side_thing(&pos, col as u8, thing3, thing2, thing1) {
                        pos = pos.exclude(col as u8, thing1.row, thing1.value);
                    }
                }
                pos
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_possibilities() {
        let p1 = Possibilities::new().exclude(0, 0, 0);
        let p2 = Possibilities::new().exclude(0, 0, 0);
        assert_eq!(p1, p2);
    }
}
