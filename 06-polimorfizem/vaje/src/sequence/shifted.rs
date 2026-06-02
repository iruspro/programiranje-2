use crate::sequence::Sequence;

pub struct ShiftedSequence<S> {
    name: String,
    sequence: S,
    shift: u64,
}

impl<S> ShiftedSequence<S> {
    pub fn new(name: &str, sequence: S, shift: u64) -> Self {
        ShiftedSequence {
            name: name.to_string(),
            sequence,
            shift,
        }
    }
}

impl<T, S> Sequence<T> for ShiftedSequence<S>
where
    T: PartialEq,
    S: Sequence<T>,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn start(&self) -> T {
        self.sequence.k_th(self.shift)
    }

    fn k_th(&self, k: u64) -> T {
        self.sequence.k_th(self.shift + k)
    }

    fn contains(&self, value: &T) -> bool {
        for i in self.shift..=u64::MAX {
            if value.eq(&self.sequence.k_th(i)) {
                return true;
            }
        }
        false
    }
}
