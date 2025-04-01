use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

fn main() {
    let seq1 = ArithmeticSequence::new(1, 1);
    let seq1 = shifted_sequence(seq1, 100);
    println!("{}", seq1.current())
}

struct ArithmeticSequence<T> {
    first: T,
    difference: T,
    current: T,
}

impl<T> ArithmeticSequence<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialOrd,
{
    fn new(first: T, difference: T) -> Self {
        ArithmeticSequence {
            first,
            difference,
            current: first,
        }
    }

    fn next(&mut self) -> T {
        let tmp = self.current;
        self.current = self.current + self.difference;
        tmp
    }

    fn n_th(&self, n: u32) -> T {
        let mut n_th_el = self.first;
        for _ in 0..n {
            n_th_el = n_th_el + self.difference
        }
        n_th_el
    }

    fn reset(&mut self) {
        self.current = self.first;
    }

    fn current(&self) -> T {
        self.current
    }

    fn sum(&self, n: u32) -> Option<T> {
        if n > 0 {
            let mut sum = self.first;
            for i in 0..(n - 1) {
                sum = sum + self.k_th(i + 1);
            }
            Some(sum)
        } else {
            None
        }
    }

    fn plus_other(&self, other: &Self) -> Self {
        ArithmeticSequence::new(self.first + other.first, self.difference + other.difference)
    }

    fn times_other(&self, other: &Self) -> Self {
        ArithmeticSequence::new(self.first * other.first, self.difference * other.difference)
    }
}

impl<T> PartialEq for ArithmeticSequence<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.difference == other.difference
    }
}

enum SequenceName {
    Constant,
    Arithmetic,
    // Geometric,
    // Fibonacci,
}

trait Sequence<T> {
    fn name(&self) -> SequenceName;
    fn start(&mut self, el: T) {}
    fn k_th(&self, n: u32) -> T;
    fn contains(&self, el: T) -> bool;
}

struct ConstantSequence<T> {
    first: T,
}

impl<T> Sequence<T> for ConstantSequence<T>
where
    T: Copy + PartialEq,
{
    fn name(&self) -> SequenceName {
        SequenceName::Constant
    }

    fn k_th(&self, n: u32) -> T {
        self.first
    }

    fn contains(&self, el: T) -> bool {
        el == self.first
    }
}

impl<T> Sequence<T> for ArithmeticSequence<T>
where
    T: Copy + Add<Output = T> + PartialOrd,
{
    fn name(&self) -> SequenceName {
        SequenceName::Arithmetic
    }

    fn start(&mut self, el: T) {
        self.first = el;
        self.current = el;
    }

    fn k_th(&self, n: u32) -> T {
        let mut el = self.first;
        for _ in 0..n {
            el = el + self.difference;
        }
        el
    }

    fn contains(&self, el: T) -> bool {
        let mut result = true;
        let mut current = self.first;

        if self.first < self.k_th(1) {
            result = loop {
                if current == el {
                    break true;
                } else if current > el {
                    break false;
                }
                current = current + self.difference;
            }
        } else if self.first > self.k_th(1) {
            result = loop {
                if current == el {
                    break true;
                } else if current < el {
                    break false;
                }
                current = current + self.difference;
            }
        } else {
            result = self.first == el;
        }
        result
    }
}

struct GeometricSequence<T> {
    first: T,
    common_ratio: T,
    current: T,
}

struct FibonacciSequence<T> {
    first: T,
    second: T,
    last: Option<T>,
    current: T,
}

fn shifted_sequence<T, S>(sequence: T, n: u32) -> T
where
    T: Sequence<S>,
{
    let mut sequence = sequence;
    match sequence.name() {
        SequenceName::Constant => sequence,
        SequenceName::Arithmetic => {
            sequence.start(sequence.k_th(n));
            sequence
        }
    }
}

enum BinaryOperation {
    Plus,
    Minus,
    Times,
}

enum Expression<T> {
    Constant(T),
    Operation(Box<Expression<T>>, BinaryOperation, Box<Expression<T>>),
}

impl<T> Expression<T>
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    fn eval(&self) -> T {
        match self {
            Expression::Constant(x) => *x,
            Expression::Operation(expr1, binop, expr2) => match binop {
                BinaryOperation::Plus => expr1.eval() + expr2.eval(),
                BinaryOperation::Minus => expr1.eval() - expr2.eval(),
                BinaryOperation::Times => expr1.eval() * expr2.eval(),
            },
        }
    }

    fn collect(&self) -> u32 {
        match self {
            Expression::Constant(_) => 1,
            Expression::Operation(expr1, _, expr2) => expr1.collect() + expr2.collect(),
        }
    }
}

impl<T: Display> ToString for Expression<T> {
    fn to_string(&self) -> String {
        match self {
            Expression::Constant(x) => format!("{x}"),
            Expression::Operation(expr1, binop, expr2) => match binop {
                BinaryOperation::Plus => format!("({} + {})", expr1.to_string(), expr2.to_string()),
                BinaryOperation::Minus => {
                    format!("({} - {})", expr1.to_string(), expr2.to_string())
                }
                BinaryOperation::Times => {
                    format!("({} * {})", expr1.to_string(), expr2.to_string())
                }
            },
        }
    }
}
