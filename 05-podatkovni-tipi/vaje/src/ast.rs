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

pub enum Expression {
    Const(u32),
    Op(Box<Expression>, BinOp, Box<Expression>),
}

impl Expression {
    pub fn plus(expr1: Expression, expr2: Expression) -> Expression {
        Expression::Op(Box::new(expr1), BinOp::Plus, Box::new(expr2))
    }

    pub fn minus(expr1: Expression, expr2: Expression) -> Expression {
        Expression::Op(Box::new(expr1), BinOp::Minus, Box::new(expr2))
    }

    pub fn times(expr1: Expression, expr2: Expression) -> Expression {
        Expression::Op(Box::new(expr1), BinOp::Times, Box::new(expr2))
    }

    pub fn eval(&self) -> u32 {
        match self {
            Self::Const(k) => *k,
            Self::Op(expr1, op, expr2) => match op {
                BinOp::Plus => expr1.eval() + expr2.eval(),
                BinOp::Minus => expr1.eval() - expr2.eval(),
                BinOp::Times => expr1.eval() * expr2.eval(),
            },
        }
    }

    pub fn collect(&self) -> u32 {
        match self {
            Self::Const(_) => 1,
            Self::Op(expr1, _, expr2) => expr1.collect() + expr2.collect(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Const(k) => k.to_string(),
            Self::Op(expr1, op, expr2) => {
                format!(
                    "({} {} {})",
                    expr1.to_string(),
                    op.to_str(),
                    expr2.to_string()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_return_correct_sum_for_two_arguments() {
        let expr = Expression::plus(Expression::Const(1), Expression::Const(2));

        let result = Expression::eval(&expr);

        assert_eq!(result, 3);
    }

    #[test]
    fn collect_return_correct_number_for_sum_of_two_constant_terms() {
        let expr = Expression::plus(Expression::Const(1), Expression::Const(2));

        let result = Expression::collect(&expr);

        assert_eq!(result, 2);
    }

    #[test]
    fn to_string_return_correct_string_for_sum_of_two_constant_terms() {
        let expr = Expression::plus(Expression::Const(1), Expression::Const(2));

        let result = Expression::to_string(&expr);

        assert_eq!(result, "(1 + 2)");
    }
}
