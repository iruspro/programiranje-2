use std::{
    collections::HashMap,
    ops::{Add, Mul, Sub},
};

use crate::{expression::Expression, sequence::Sequence};

pub struct CombinedSequence<'a, T> {
    name: String,
    expression: Expression<T>,
    sequences: Vec<&'a dyn Sequence<T>>,
}

impl<'a, T> CombinedSequence<'a, T> {
    pub fn new(
        name: impl Into<String>,
        expression: Expression<T>,
        sequences: Vec<&'a dyn Sequence<T>>,
    ) -> Self {
        CombinedSequence {
            name: name.into(),
            expression,
            sequences,
        }
    }
}

impl<'a, T> Sequence<T> for CombinedSequence<'a, T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn start(&self) -> T {
        self.expression
            .eval(&|var| {
                for seq in &self.sequences {
                    if seq.name() == var {
                        return seq.start();
                    }
                }
                panic!("")
            })
            .expect("msg")
    }

    fn k_th(&self, k: u64) -> T {
        self.expression
            .eval(&|var| {
                for seq in &self.sequences {
                    if seq.name() == var {
                        return seq.k_th(k);
                    }
                }
                panic!("")
            })
            .expect("msg")
    }

    fn contains(&self, value: &T) -> bool {
        todo!()
    }
}
