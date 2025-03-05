#[derive(Debug)]
struct ArithmeticSequence {
    first_term: i32,
    common_difference: i32,
    current_term: i32,
}


impl ArithmeticSequence {
    fn new(first_term: i32, common_difference: i32) -> Self {
        Self {
            first_term,
            common_difference,
            current_term: first_term,
        }        
    }

    fn next(&mut self) -> i32 {
        let current_term = self.current_term;
        self.current_term += self.common_difference;
        current_term
    }
    
    fn n_th(&self, n: i32) -> i32 {
        let n_th_term = self.first_term + (n - 1) * self.common_difference;
        n_th_term
    }

    fn reset(&mut self) {
        self.current_term = self.first_term;
    }

    fn current(&self) -> i32 {
        self.current_term
    }

    fn sum(&self, n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += self.n_th(i)
        };
        sum
    }

    fn plus_other(&self, other: Self) -> Self {
        Self {
            first_term: self.first_term + other.first_term,
            common_difference: self.common_difference + other.common_difference,
            current_term: self.first_term + other.first_term,
        }
    }

    fn product_other(&self, other: Self) -> Self {
        Self {
            first_term: self.first_term * other.first_term,
            common_difference: self.common_difference + other.common_difference,
            current_term: self.first_term * other.first_term,
        }
    }
}


fn main() {
    let mut a = ArithmeticSequence::new(1, 1);
    let mut b = ArithmeticSequence::new(1, 1);
    let c = a.plus_other(b);
    println!("{}", c.sum(100));
}
