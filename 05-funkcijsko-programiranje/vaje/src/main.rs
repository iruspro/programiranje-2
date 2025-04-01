use std::fmt::Display;

// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
fn apply_int<F>(f: F, x: i32) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(x)
}

//  apply: ('a -> 'b) -> 'a -> 'b
fn apply<F, S, T>(f: F, x: S) -> T
where
    F: FnOnce(S) -> T,
{
    f(x)
}

//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
fn apply2<F, S, T>(f: F, x: S, y: S) -> T 
where 
    F: FnOnce(S, S) -> T 
{
    f(x, y)
}

//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, funkcijo narediti najbolj splošno)
fn map<F, S, T>(mut f: F, v: Vec<S>) -> Vec<T>
where 
    F: FnMut(S) -> T,
{
    let mut res: Vec<T> = Vec::new();
    for el in v {
        res.push(f(el));
    }
    res
}

//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
fn repeat<F, T>(n: u32, mut f: F, x: T) -> T
where
    F: FnMut(T) -> T
{
    let mut res = f(x);
    for _ in 0..(n - 1) {
        res = f(res);
    }
    res
}

//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter
fn filter<F, T>(mut f: F, v: Vec<T>) -> Vec<T>
where 
    F: FnMut(&T) -> bool,
    T: Clone
{
    v.into_iter().filter(f).collect()
}

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W
// števil in izpiše vsako v svojo vrstico
fn print_xyz(v: Vec<i32>) {
    v.iter().for_each(|x| println!("{x}"));
}
// nizov in izpiše njihove dolžine
fn print_len(v: Vec<String>) {
    v.iter().for_each(|s| println!("{}", s.len()));
}
// nizov in vrne vsoto njihovih dolžin
fn get_sum_len(v: Vec<String>) -> usize {
    v.iter().map(|s| s.len()).sum()
}
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
fn get_positive_product_sum(v: Vec<(i32, i32)>) -> i32 {
    v.iter().map(|(x, y)| x * y).filter(|x| x > &0).sum()
}
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
fn vec_sum(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    v1.iter().zip(v2.iter()).map(|(x, y)| x + y).collect()
}
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov
fn vec_positive_product_sum(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    v1.iter().zip(v2.iter()).map(|(x, y)| x * y).filter(|x| x > &0).sum()
}
// vektor Option<T> in izpiše vse T-je
fn print_t<T: Display>(v: Vec<Option<T>>) {
    v.iter().for_each(
        |x| {
            match x {
                Some(x) => println!("{x}"),
                None => ()
            }
        }
    );
}
// vektor Option<T> in vrne število Some-ov
fn number_some<T>(v: Vec<Option<T>>) -> usize {
    let mut count = 0;
    v.iter().for_each(
        |x| {
            match x {
                Some(_) => count += 1,
                None => ()
            }
        }
    );
    count
}
// odfiltrira števila deljiva s 3

// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     panic!("Not implemented");
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/

fn main() {}
