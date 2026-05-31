mod ast;
mod sequences;

use crate::sequences::{arithmetic::ArithmeticSequence, utils};
use ast::Expression;

fn main() {
    let seq1 = ArithmeticSequence::new(1, 1);
    let seq2 = ArithmeticSequence::new(2, 2);
    let _seq3 = utils::sum(&seq1, &seq2);

    let expr1 = Expression::plus(
        Expression::Const(1),
        Expression::times(Expression::Const(2), Expression::Const(3)),
    );

    let expr2 = Expression::times(
        Expression::plus(Expression::Const(1), Expression::Const(2)),
        Expression::Const(3),
    );

    let expr3 = Expression::plus(
        Expression::plus(Expression::Const(1), Expression::Const(2)),
        Expression::Const(3),
    );

    let expr4 = Expression::plus(
        Expression::times(Expression::Const(5), Expression::Const(5)),
        Expression::times(Expression::Const(3), Expression::Const(3)),
    );

    let expr5 = Expression::plus(
        Expression::times(Expression::Const(5), Expression::Const(5)),
        Expression::times(Expression::Const(4), Expression::Const(4)),
    );

    println!("{} = {}", expr1.to_string(), expr1.eval());
    println!("{} = {}", expr2.to_string(), expr2.eval());
    println!("{} = {}", expr3.to_string(), expr3.eval());
    println!("{} = {}", expr4.to_string(), expr4.eval());
    println!("{} = {}", expr5.to_string(), expr5.eval());

    println!("const in expr1: {}", expr1.collect());
    println!("const in expr2: {}", expr2.collect());
    println!("const in expr3: {}", expr3.collect());
    println!("const in expr4: {}", expr4.collect());
    println!("const in expr5: {}", expr5.collect());
}
