use std::ops::{Add, Mul};

use crate::sequence::Sequence;

pub struct ArithmeticSequence<T> {
    name: String,

    init: T,
    diff: T,
}

impl<T> ArithmeticSequence<T> {
    pub fn new(name: impl Into<String>, initial_term: T, common_difference: T) -> Self {
        ArithmeticSequence {
            name: name.into(),
            init: initial_term,
            diff: common_difference,
        }
    }
}

impl<'a, T> Sequence<T> for ArithmeticSequence<T>
where
    T: Clone + PartialOrd + Add<T, Output = T>,
{
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn start(&self) -> T {
        self.init.clone()
    }

    fn k_th(&self, k: u64) -> T {
        if k == 0 {
            panic!("invalid index k = 0")
        }

        let mut result = self.init.clone();
        for _ in 0..k {
            result = result + self.diff.clone();
        }

        result
    }

    fn contains(&self, value: &T) -> bool {
        let first = self.k_th(1);
        let second = self.k_th(2);
        let asc = first <= second;

        for k in 1..=u64::MAX {
            let c = self.k_th(k);
            if asc && c.gt(value) {
                return false;
            } else if !asc && c.lt(value) {
                return false;
            } else {
                if c.eq(value) {
                    return true;
                }
            }
        }
        false
    }
}

impl<T: PartialEq> PartialEq for ArithmeticSequence<T> {
    fn eq(&self, other: &Self) -> bool {
        self.init == other.init && self.diff == other.diff
    }
}

impl<T> Mul for ArithmeticSequence<T>
where
    T: Clone + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ArithmeticSequence::new(
            self.name.as_str(),
            (self.init * rhs.init.clone()),
            (self.diff * rhs.diff.clone()),
        )
    }
}
