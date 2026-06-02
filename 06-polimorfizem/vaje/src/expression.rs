use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

enum BinOp {
    Plus,
    Minus,
    Times,
}

impl BinOp {
    fn to_str(&self) -> &str {
        match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Times => "*",
        }
    }
}

pub enum Expression<T> {
    Const(T),
    Variable(String),
    Op(Box<Expression<T>>, BinOp, Box<Expression<T>>),
}

impl<T> Expression<T> {
    pub fn c(c: T) -> Expression<T> {
        Expression::Const(c)
    }

    pub fn var(name: String) -> Expression<T> {
        Expression::Variable(name)
    }

    pub fn plus(expr1: Expression<T>, expr2: Expression<T>) -> Expression<T> {
        Expression::Op(Box::new(expr1), BinOp::Plus, Box::new(expr2))
    }

    pub fn minus(expr1: Expression<T>, expr2: Expression<T>) -> Expression<T> {
        Expression::Op(Box::new(expr1), BinOp::Minus, Box::new(expr2))
    }

    pub fn times(expr1: Expression<T>, expr2: Expression<T>) -> Expression<T> {
        Expression::Op(Box::new(expr1), BinOp::Times, Box::new(expr2))
    }

    pub fn collect(&self) -> u32 {
        match self {
            Self::Const(_) => 1,
            Self::Variable(_) => 0,
            Self::Op(expr1, _, expr2) => expr1.collect() + expr2.collect(),
        }
    }
}

impl<T: Display> Display for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(k) => write!(f, "{k}"),
            Self::Variable(x) => write!(f, "{x}"),
            Self::Op(expr1, op, expr2) => {
                write!(f, "({} {} {})", expr1, op.to_str(), expr2)
            }
        }
    }
}

impl<T> Expression<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn eval<F>(&self, vars: &F) -> Result<T, &str>
    where
        F: Fn(&str) -> T,
    {
        match self {
            Self::Const(k) => Ok(k.clone()),
            Self::Variable(x) => Ok(vars(x)),
            Self::Op(expr1, op, expr2) => {
                let a = expr1.eval(vars)?;
                let b = expr2.eval(vars)?;
                match op {
                    BinOp::Plus => Ok(a + b),
                    BinOp::Minus => Ok(a - b),
                    BinOp::Times => Ok(a * b),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_return_correct_sum_for_two_integers() {
        let expr = Expression::plus(Expression::Const(1), Expression::Const(2));

        let result = Expression::eval(&expr, &|_| panic!("no vars")).unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn collect_return_correct_number_for_sum_of_two_constant_terms() {
        let expr = Expression::plus(Expression::Const(""), Expression::Const("a"));

        let result = Expression::collect(&expr);

        assert_eq!(result, 2);
    }

    #[test]
    fn to_string_return_correct_string_for_sum_of_two_integers() {
        let expr = Expression::plus(Expression::Const(1), Expression::Const(2));

        let result = Expression::to_string(&expr);

        assert_eq!(result, "(1 + 2)");
    }
}
