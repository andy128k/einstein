use crate::bitset::BitSet;
use crate::u4::U4;
use crate::util::{converge::converge, num_to_str::num_to_str};
use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    Rng,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::convert::TryInto;
use std::fmt;

fn only<T>(values: &[T]) -> Option<&T> {
    if values.len() == 1 {
        values.first()
    } else {
        None
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PuzzleSize {
    pub kinds: u8,
    pub values: u8,
}

impl PuzzleSize {
    pub fn iter_kinds(&self) -> impl Iterator<Item = Kind> {
        (0..self.kinds).map(Kind)
    }

    pub fn iter_values(&self) -> impl Iterator<Item = Value> {
        (0..self.kinds).map(Value)
    }
}

impl Default for PuzzleSize {
    fn default() -> Self {
        Self {
            kinds: 6,
            values: 6,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize, Hash)]
pub struct Kind(pub u8);

impl Kind {
    fn name(&self) -> Cow<'static, str> {
        match self.0 {
            0 => "N".into(),
            1 => "L".into(),
            2 => "R".into(),
            3 => "D".into(),
            4 => "G".into(),
            5 => "M".into(),
            n => num_to_str(n - 6, &["X", "Y", "Z"]).into(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize, Hash)]
pub struct Value(pub u8);

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Thing {
    pub row: Kind,
    pub value: Value,
}

impl Thing {
    fn name(&self) -> String {
        format!("{}{}", self.row.name(), self.value.0 + 1)
    }
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.row.0 == 0 {
            return write!(f, "{}", self.value.0 + 1);
        }
        const LABELS: &[&[&str]] = &[
            &["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"],
            &["Ⅰ", "Ⅱ", "Ⅲ", "Ⅳ", "Ⅴ", "Ⅵ", "Ⅶ", "Ⅷ", "Ⅸ", "Ⅹ", "Ⅺ", "Ⅻ"],
            &["⚀", "⚁", "⚂", "⚃", "⚄", "⚅", "🁖", "🁗", "🁘", "🁙"],
            &[
                "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ", "ν", "ξ",
            ],
            &[
                "+", "−", "×", "÷", "=", "√", "<", ">", "≠", "≤", "≥", "±", "±", "%",
            ],
        ];
        if let Some(label) = LABELS
            .get(usize::from(self.row.0) - 1)
            .and_then(|alphabet| alphabet.get(usize::from(self.value.0)))
        {
            write!(f, "{}", label)
        } else {
            write!(f, "{}", self.name())
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct KindMap<T>(Vec<T>);

impl<T: Clone> KindMap<T> {
    fn new(kinds: u8, value: T) -> Self {
        Self(vec![value; usize::from(kinds)])
    }

    fn set(&mut self, kind: Kind, value: T) {
        self.0[usize::from(kind.0)] = value;
    }

    fn get(&self, kind: Kind) -> &T {
        &self.0[usize::from(kind.0)]
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SolvedPuzzle {
    size: PuzzleSize,
    values: KindMap<Vec<Value>>,
}

impl SolvedPuzzle {
    pub fn random(size: PuzzleSize, rng: &mut impl Rng) -> Self {
        let mut values = KindMap::new(size.kinds, Vec::new());
        for kind in size.iter_kinds() {
            let mut kvalues: Vec<Value> = size.iter_values().collect();
            kvalues.shuffle(rng);

            values.set(kind, kvalues);
        }
        Self { size, values }
    }

    pub fn size(&self) -> PuzzleSize {
        self.size
    }

    pub fn get_value(&self, row: Kind, col: u8) -> Value {
        self.values.get(row)[usize::from(col)]
    }

    pub fn get(&self, row: Kind, col: u8) -> Thing {
        Thing {
            row,
            value: self.get_value(row, col),
        }
    }
}

fn value_to_u4(value: Value) -> U4 {
    value.0.try_into().unwrap()
}

fn u4_to_value(num: U4) -> Value {
    Value(num.into())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ValueSet(BitSet);

impl ValueSet {
    pub fn empty() -> Self {
        Self(BitSet::empty())
    }

    pub fn full(count: u8) -> Self {
        Self(BitSet::full(count))
    }

    pub fn single(value: Value) -> Self {
        Self(BitSet::empty().with(value_to_u4(value)))
    }

    pub fn iter(&self) -> impl Iterator<Item = Value> {
        self.0.iter().map(u4_to_value)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn get_single(&self) -> Option<Value> {
        self.0.get_single().map(u4_to_value)
    }

    pub fn contains(&self, value: Value) -> bool {
        self.0.contains(value_to_u4(value))
    }

    pub fn add(&mut self, value: Value) {
        self.0.add(value_to_u4(value));
    }

    pub fn remove(&mut self, value: Value) -> bool {
        self.0.remove(value_to_u4(value))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct PossibilitiesRow(Vec<ValueSet>);

impl PossibilitiesRow {
    fn new(values: u8) -> Self {
        Self(vec![ValueSet::full(values); values as usize])
    }

    fn columns(&self) -> usize {
        self.0.len()
    }

    fn exclude(&self, col: u8, element: Value) -> Self {
        let mut new_row = self.clone();
        new_row.0[usize::from(col)].remove(element);
        new_row.check_singles()
    }

    fn set(&self, col: u8, element: Value) -> Self {
        let mut new_row = self.clone();
        new_row.0[usize::from(col)] = ValueSet::single(element);
        new_row.check_singles()
    }

    fn value_in_columns(&self, value: Value) -> Vec<u8> {
        (0..self.columns())
            .filter(|col| self.0[*col].contains(value))
            .map(|col| col as u8)
            .collect()
    }

    fn check_single_value_in_a_cell(&self) -> Self {
        let mut new_row = self.clone();
        for col in 0..self.columns() {
            if let Some(value) = self.0[col].get_single() {
                // there is only one element in cell but it used somewhere else
                for i in 0..self.columns() {
                    if i != col {
                        new_row.0[i].remove(value);
                    }
                }
            }
        }
        new_row
    }

    fn check_single_value_in_a_row(&self) -> Self {
        let mut new_row = self.clone();
        // check for single element without exclusive cell
        for value_index in 0..self.columns() {
            let value = Value(value_index as u8);
            if let Some(col) = only(&new_row.value_in_columns(value)) {
                new_row.0[*col as usize] = ValueSet::single(value);
            }
        }
        new_row
    }

    fn check_singles(&self) -> Self {
        converge(self.clone(), |p| {
            p.check_single_value_in_a_cell()
                .check_single_value_in_a_row()
        })
    }

    fn is_solved(&self) -> bool {
        self.0.iter().all(|s| s.size() == 1)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Possibilities {
    size: PuzzleSize,
    rows: KindMap<PossibilitiesRow>,
}

impl Possibilities {
    pub fn new(size: PuzzleSize) -> Self {
        Self {
            size,
            rows: KindMap::new(size.kinds, PossibilitiesRow::new(size.values)),
        }
    }

    pub fn exclude(&self, col: u8, kind: Kind, value: Value) -> Self {
        let row = self.rows.get(kind).exclude(col, value);

        let mut new = self.clone();
        new.rows.set(kind, row);
        new
    }

    pub fn set(&self, col: u8, kind: Kind, value: Value) -> Self {
        let row = self.rows.get(kind).set(col, value);

        let mut new = self.clone();
        new.rows.set(kind, row);
        new
    }

    pub fn get_possible(&self, col: u8, row: Kind) -> ValueSet {
        self.rows.get(row).0[usize::from(col)]
    }

    pub fn is_possible(&self, col: u8, thing: Thing) -> bool {
        self.get_possible(col, thing.row).contains(thing.value)
    }

    pub fn is_defined(&self, col: u8, row: Kind) -> bool {
        self.get_possible(col, row).size() == 1
    }

    pub fn get_defined(&self, col: u8, row: Kind) -> Option<Value> {
        self.get_possible(col, row).get_single()
    }

    pub fn is_solved(&self) -> bool {
        self.size
            .iter_kinds()
            .all(|kind| self.rows.get(kind).is_solved())
    }

    pub fn is_valid(&self, puzzle: &SolvedPuzzle) -> bool {
        for kind in self.size.iter_kinds() {
            for col in 0..self.size.values {
                let thing = puzzle.get(kind, col);
                if !self.get_possible(col, kind).contains(thing.value) {
                    return false;
                }
            }
        }
        true
    }
}

fn gen_two(rng: &mut impl Rng, max: u8) -> (u8, u8) {
    let v1: u8 = rng.gen_range(0..(max - 1));
    let v2: u8 = rng.gen_range((v1 + 1)..max);
    (v1, v2)
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Rule {
    Near(Thing, Thing),
    Direction(Thing, Thing),
    Open(u8, Thing), // column
    Under(Thing, Thing),
    Between(Thing, Thing, Thing),
}

impl Rule {
    pub fn is_non_trivial(self) -> bool {
        match self {
            Rule::Open(..) => false,
            _ => true,
        }
    }
}

fn generate_near_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let size = puzzle.size();
    let row1 = rng.gen_range(0..size.kinds);
    let row2 = rng.gen_range(0..size.kinds);
    let first_col = rng.gen_range(0..(size.values - 1));

    let thing1 = puzzle.get(Kind(row1), first_col);
    let thing2 = puzzle.get(Kind(row2), first_col + 1);

    if rng.gen() {
        Rule::Near(thing1, thing2)
    } else {
        Rule::Near(thing2, thing1)
    }
}

fn generate_direction_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let size = puzzle.size();
    let row1 = rng.gen_range(0..size.kinds);
    let row2 = rng.gen_range(0..size.kinds);
    let (col1, col2) = gen_two(rng, size.values);

    let thing1 = puzzle.get(Kind(row1), col1);
    let thing2 = puzzle.get(Kind(row2), col2);
    Rule::Direction(thing1, thing2)
}

fn generate_open_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let size = puzzle.size();
    let row = rng.gen_range(0..size.kinds);
    let col = rng.gen_range(0..size.values);

    let thing = puzzle.get(Kind(row), col);
    Rule::Open(col, thing)
}

fn generate_under_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let size = puzzle.size();
    let col: u8 = rng.gen_range(0..size.values);
    let (row1, row2) = gen_two(rng, size.kinds);

    let thing1 = puzzle.get(Kind(row1), col);
    let thing2 = puzzle.get(Kind(row2), col);
    Rule::Under(thing1, thing2)
}

fn generate_between_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let size = puzzle.size();
    let row1 = rng.gen_range(0..size.kinds);
    let row2 = rng.gen_range(0..size.kinds);
    let row3 = rng.gen_range(0..size.kinds);
    let first_col = rng.gen_range(0..(size.values - 2));

    let thing1 = puzzle.get(Kind(row1), first_col);
    let thing2 = puzzle.get(Kind(row2), first_col + 1);
    let thing3 = puzzle.get(Kind(row3), first_col + 2);

    if rng.gen() {
        Rule::Between(thing1, thing2, thing3)
    } else {
        Rule::Between(thing3, thing2, thing1)
    }
}

pub fn generate_rule(rng: &mut impl Rng, puzzle: &SolvedPuzzle) -> Rule {
    let weights = [4_u32, 1, 2, 4, 3];
    let dist = WeightedIndex::new(&weights).unwrap();
    match dist.sample(rng) {
        0 => generate_near_rule(rng, puzzle),
        1 => generate_open_rule(rng, puzzle),
        2 => generate_under_rule(rng, puzzle),
        3 => generate_direction_rule(rng, puzzle),
        4 => generate_between_rule(rng, puzzle),
        _ => unreachable!(),
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Near(thing1, thing2) => write!(f, "{} is near to {}", thing1, thing2),
            Rule::Direction(thing1, thing2) => {
                write!(f, "{} is from the left of {}", thing1, thing2)
            }
            Rule::Open(col, thing) => write!(f, "{} is at column {}", thing, col + 1),
            Rule::Under(thing1, thing2) => write!(f, "{} is the same column as {}", thing1, thing2),
            Rule::Between(thing1, thing2, thing3) => {
                write!(f, "{} is between {} and {}", thing2, thing1, thing3)
            }
        }
    }
}

pub fn apply(pos: &Possibilities, rule: &Rule) -> Possibilities {
    match *rule {
        Rule::Near(thing1, thing2) => {
            fn is_applicable_to_col(
                pos: &Possibilities,
                col: u8,
                thing: Thing,
                neighbour: Thing,
            ) -> bool {
                let has_left = if col == 0 {
                    false
                } else {
                    pos.is_possible(col - 1, neighbour)
                };

                let has_right = if col + 1 == pos.size.values {
                    false
                } else {
                    pos.is_possible(col + 1, neighbour)
                };

                !has_right && !has_left && pos.is_possible(col, thing)
            }
            converge(pos.clone(), |mut pos| {
                for col in 0..pos.size.values {
                    if is_applicable_to_col(&pos, col, thing1, thing2) {
                        pos = pos.exclude(col, thing1.row, thing1.value);
                    }
                    if is_applicable_to_col(&pos, col, thing2, thing1) {
                        pos = pos.exclude(col, thing2.row, thing2.value);
                    }
                }
                pos
            })
        }
        Rule::Direction(thing1, thing2) => {
            let mut new_pos = pos.clone();
            for col in 0..pos.size.values {
                if new_pos.is_possible(col, thing2) {
                    new_pos = new_pos.exclude(col, thing2.row, thing2.value);
                }
                if new_pos.is_possible(col, thing1) {
                    break;
                }
            }
            for col in (0..pos.size.values).rev() {
                if new_pos.is_possible(col, thing1) {
                    new_pos = new_pos.exclude(col, thing1.row, thing1.value);
                }
                if new_pos.is_possible(col, thing2) {
                    break;
                }
            }
            new_pos
        }
        Rule::Open(col, thing) => pos.set(col, thing.row, thing.value),
        Rule::Under(thing1, thing2) => {
            let mut new_pos = pos.clone();
            for col in 0..pos.size.values {
                if !new_pos.is_possible(col, thing1) {
                    new_pos = new_pos.exclude(col, thing2.row, thing2.value);
                }
                if !new_pos.is_possible(col, thing2) {
                    new_pos = new_pos.exclude(col, thing1.row, thing1.value);
                }
            }
            new_pos
        }
        Rule::Between(thing1, thing2, thing3) => {
            fn check_middle_thing(
                pos: &Possibilities,
                col: u8,
                thing1: Thing,
                thing2: Thing,
                thing3: Thing,
            ) -> bool {
                col > 0
                    && col < pos.size.values - 1
                    && pos.is_possible(col, thing2)
                    && ((pos.is_possible(col - 1, thing1) && pos.is_possible(col + 1, thing3))
                        || (pos.is_possible(col - 1, thing3) && pos.is_possible(col + 1, thing1)))
            }

            fn check_side_thing(
                pos: &Possibilities,
                col: u8,
                thing1: Thing,
                thing2: Thing,
                thing3: Thing,
            ) -> bool {
                if pos.is_possible(col, thing3) {
                    let left_possible = if col < 2 {
                        false
                    } else {
                        pos.is_possible(col - 1, thing2) && pos.is_possible(col - 2, thing1)
                    };
                    let right_possible = if col >= pos.size.values - 2 {
                        false
                    } else {
                        pos.is_possible(col + 1, thing2) && pos.is_possible(col + 2, thing1)
                    };
                    left_possible || right_possible
                } else {
                    false
                }
            }

            converge(pos.clone(), |mut pos| {
                for col in 0..pos.size.values {
                    if !check_middle_thing(&pos, col, thing1, thing2, thing3) {
                        pos = pos.exclude(col, thing2.row, thing2.value);
                    }
                    if !check_side_thing(&pos, col, thing1, thing2, thing3) {
                        pos = pos.exclude(col, thing3.row, thing3.value);
                    }
                    if !check_side_thing(&pos, col, thing3, thing2, thing1) {
                        pos = pos.exclude(col, thing1.row, thing1.value);
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
        let size = PuzzleSize {
            kinds: 6,
            values: 6,
        };
        let p1 = Possibilities::new(size).exclude(0, Kind(0), Value(0));
        let p2 = Possibilities::new(size).exclude(0, Kind(0), Value(0));
        assert_eq!(p1, p2);
    }
}
