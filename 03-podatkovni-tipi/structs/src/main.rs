#[derive(Debug)]
struct ArithmeticSequence {
    first: i32,
    difference: i32,
    current: i32,
}

impl ArithmeticSequence {
    fn new(first: i32, difference: i32) -> Self {
        ArithmeticSequence {
            first,
            difference,
            current: first,
        }
    }

    fn next(&mut self) -> i32 {
        let tmp = self.current;
        self.current += self.difference;
        tmp
    }

    fn n_th(&self, n: u32) -> i32 {
        let n: i32 = n.try_into().unwrap();
        self.first + n * self.difference
    }

    fn reset(&mut self) {
        self.current = self.first;
    }

    fn current(&self) -> i32 {
        self.current
    }

    fn sum(&self, n: u32) -> i32 {
        let mut sum = 0;
        for i in 0..n {
            sum += self.n_th(i);
        }
        sum
    }

    fn plus_other(&self, other: &Self) -> Self {
        ArithmeticSequence::new(self.first + other.first, self.difference + other.difference)
    }

    fn times_other(&self, other: &Self) -> Self {
        ArithmeticSequence::new(self.first * other.first, self.difference * other.difference)
    }
}

struct GeometricSequence {
    first: i32,
    common_ratio: i32,
    current: i32,
}

impl GeometricSequence {
    fn new(first: i32, common_ratio: i32) -> Self {
        GeometricSequence {
            first,
            common_ratio,
            current: first,
        }
    }

    fn next(&mut self) -> i32 {
        let tmp = self.current;
        self.current *= self.common_ratio;
        tmp
    }

    fn n_th(&self, n: u32) -> i32 {
        self.first * self.common_ratio.pow(n)
    }

    fn reset(&mut self) {
        self.current = self.first;
    }

    fn current(&self) -> i32 {
        self.current
    }

    fn sum(&self, n: u32) -> i32 {
        let mut sum = 0;
        for i in 0..n {
            sum += self.n_th(i);
        }
        sum
    }

    fn plus_other(&self, other: Self) -> Self {
        GeometricSequence::new(self.first + other.first, self.common_ratio + other.common_ratio)
    }

    fn times_other(&self, other: Self) -> Self {
        GeometricSequence::new(self.first * other.first, self.common_ratio * other.common_ratio)
    }
}

enum BinaryOperation {
    Plus,
    Minus,
    Times,
}

enum Expression {
    Constant(u32),
    Operation(Box<Expression>, BinaryOperation, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> u32 {
        match self {
            Expression::Constant(x) => *x,
            Expression::Operation(expr1, binop, expr2 ) => {
                match binop {
                    BinaryOperation::Plus => expr1.eval() + expr2.eval(),
                    BinaryOperation::Minus => expr1.eval() - expr2.eval(),
                    BinaryOperation::Times => expr1.eval() * expr2.eval(),
                }
            }
        }
    }

    fn collect(&self) -> u32 {
        match self {
            Expression::Constant(_) => 1,
            Expression::Operation(expr1, _, expr2) => {
                expr1.collect() + expr2.collect()
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::Constant(x) => format!("{x}"),
            Expression::Operation(expr1, binop, expr2) => {
                match binop {
                    BinaryOperation::Plus => format!("({} + {})", expr1.to_string(), expr2.to_string()),
                    BinaryOperation::Minus => format!("({} - {})", expr1.to_string(), expr2.to_string()),
                    BinaryOperation::Times => format!("({} * {})", expr1.to_string(), expr2.to_string()),
                }
            }
        }
    }
}

fn main() {
    let expr1 = Expression::Operation(
        Box::new(Expression::Constant(1)), 
        BinaryOperation::Plus, 
        Box::new(Expression::Operation(Box::new(Expression::Constant(2)), BinaryOperation::Times, Box::new(Expression::Constant(3))))
    );
    let expr2 = Expression::Operation(
        Box::new(Expression::Operation(Box::new(Expression::Constant(1)), BinaryOperation::Plus, Box::new(Expression::Constant(2)))), 
        BinaryOperation::Times,
        Box::new(Expression::Constant(3))
    );
    let expr3 = Expression::Operation(
        Box::new(Expression::Operation(Box::new(Expression::Constant(1)), BinaryOperation::Plus, Box::new(Expression::Constant(2)))), 
        BinaryOperation::Plus,
        Box::new(Expression::Constant(3))    
    );
    let expr4 = Expression::Operation(
        Box::new(Expression::Operation(Box::new(Expression::Constant(5)), BinaryOperation::Times, Box::new(Expression::Constant(5)))), 
        BinaryOperation::Plus, 
        Box::new(Expression::Operation(Box::new(Expression::Constant(3)), BinaryOperation::Times, Box::new(Expression::Constant(3))))
    );
    let expr5 = Expression::Operation(
        Box::new(Expression::Operation(Box::new(Expression::Constant(5)), BinaryOperation::Times, Box::new(Expression::Constant(5)))), 
        BinaryOperation::Plus, 
        Box::new(Expression::Operation(Box::new(Expression::Constant(4)), BinaryOperation::Times, Box::new(Expression::Constant(4))))
    );

    println!("{} {} {} {} {}", expr1.eval(), expr2.eval(), expr3.eval(), expr4.eval(), expr5.eval());
    println!("{} {} {} {} {}", expr1.collect(), expr2.collect(), expr3.collect(), expr4.collect(), expr5.collect());
    println!("{}", expr1.to_string());
    println!("{}", expr2.to_string());
    println!("{}", expr3.to_string());
    println!("{}", expr4.to_string());
    println!("{}", expr5.to_string());
}