mod expression;
mod sequence;

use sequence::arithmetic::ArithmeticSequence;
use sequence::Sequence;

fn main() {
    let seq = ArithmeticSequence::new("seq", 1, 1);
    println!("{}", seq.k_th(10))
}
