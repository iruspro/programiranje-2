use crate::sequences::arithmetic::ArithmeticSequence;

pub fn sum(seq1: &ArithmeticSequence, seq2: &ArithmeticSequence) -> ArithmeticSequence {
    ArithmeticSequence::new(
        seq1.initial_term() + seq2.initial_term(),
        seq1.common_difference() + seq2.common_difference(),
    )
}
