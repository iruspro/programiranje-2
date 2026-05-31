pub struct ArithmeticSequence {
    init: i32,
    diff: i32,

    current: i32,
}

impl ArithmeticSequence {
    pub fn new(initial_term: i32, common_difference: i32) -> Self {
        ArithmeticSequence {
            init: initial_term,
            diff: common_difference,
            current: initial_term,
        }
    }

    pub fn initial_term(&self) -> i32 {
        self.init
    }

    pub fn common_difference(&self) -> i32 {
        self.diff
    }

    pub fn next(&mut self) -> i32 {
        let current = self.current;

        self.current += self.diff;
        current
    }

    pub fn n_th(&self, n: u32) -> i32 {
        self.init + (n as i32) * self.diff
    }

    pub fn reset(&mut self) {
        self.current = self.init;
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn sum(&self, n: u32) -> i32 {
        let mut sum = 0;

        for i in 0..n {
            sum += self.n_th(i)
        }

        sum
    }
}
