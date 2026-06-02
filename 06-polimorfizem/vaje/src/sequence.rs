pub mod arithmetic;
pub mod combined;
pub mod constant;
pub mod shifted;

pub trait Sequence<T> {
    fn name(&self) -> &str;
    fn start(&self) -> T;
    fn k_th(&self, k: u64) -> T;
    fn contains(&self, value: &T) -> bool;
}
