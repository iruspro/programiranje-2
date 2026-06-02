use crate::sequence::Sequence;

pub struct ConstantSequence<T> {
    name: String,
    c: T,
}

impl<T> ConstantSequence<T> {
    fn new(name: impl Into<String>, c: T) -> Self {
        ConstantSequence {
            name: name.into(),
            c: c,
        }
    }
}

impl<T: Clone> Sequence<T> for ConstantSequence<T>
where
    T: PartialEq,
{
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn start(&self) -> T {
        self.c.clone()
    }

    fn k_th(&self, _: u64) -> T {
        self.c.clone()
    }

    fn contains(&self, value: &T) -> bool {
        self.c.eq(value)
    }
}
