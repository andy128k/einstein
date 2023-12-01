use crate::u4::U4;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::iter::successors;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct BitSet(u16);

impl BitSet {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn full(count: u8) -> Self {
        assert!(count > 0 && count <= 16);
        Self(u16::MAX >> (16 - count))
    }

    pub fn iter(&self) -> impl Iterator<Item = U4> {
        let value = self.0;
        successors(Some(1_u16), |n| n.checked_mul(2))
            .enumerate()
            .filter(move |(_index, mask)| (value & mask) != 0)
            .filter_map(|(index, _mask)| U4::try_from(index).ok())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.count_ones() == 0
    }

    pub fn get_single(&self) -> Option<U4> {
        let mut result = None;
        for value in self.iter() {
            if result.is_some() {
                return None;
            }
            result = Some(value);
        }
        result
    }

    #[inline]
    pub fn contains(self, value: U4) -> bool {
        (self.0 & mask(value)) != 0
    }

    #[inline]
    pub fn add(&mut self, value: U4) {
        self.0 |= mask(value);
    }

    #[inline]
    pub fn with(self, value: U4) -> Self {
        let mut w: Self = self;
        w.add(value);
        w
    }

    #[inline]
    pub fn remove(&mut self, value: U4) -> bool {
        let present = self.contains(value);
        self.0 &= !mask(value);
        present
    }

    #[inline]
    pub fn without(self, value: U4) -> Self {
        let mut w: Self = self;
        w.remove(value);
        w
    }
}

#[inline]
fn mask(value: U4) -> u16 {
    let shift: u8 = value.into();
    1_u16 << shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        assert_eq!(
            BitSet::full(16)
                .iter()
                .map(|v| v.into())
                .collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        assert_eq!(
            BitSet::full(15)
                .iter()
                .map(|v| v.into())
                .collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
        );
        assert_eq!(
            BitSet::full(10)
                .iter()
                .map(|v| v.into())
                .collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            BitSet::full(2)
                .iter()
                .map(|v| v.into())
                .collect::<Vec<u8>>(),
            vec![0, 1]
        );
        assert_eq!(
            BitSet::full(1)
                .iter()
                .map(|v| v.into())
                .collect::<Vec<u8>>(),
            vec![0]
        );
    }
}
