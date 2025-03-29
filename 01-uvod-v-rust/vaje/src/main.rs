use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let mut last: u32 = a0;
    let mut current: u32 = a1;
    let mut counter = 1;
    while counter < n {
        let tmp = last;
        last = current;
        current = tmp + last;
        counter += 1;
    }
    current
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno
use std::collections::HashSet;

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn number_of_days_in_month(month: u32, year: u32) -> u32 {
    let months31 = HashSet::from([1, 3, 5, 7, 8, 10, 12]);
    let months30 = HashSet::from([4, 7, 9, 11]);
    if month == 2 {
        if is_leap(year) {
            return 29;
        } else {
            return 28;
        }
    } else if months30.contains(&month) {
        return 30;
    } else if months31.contains(&month) {
        return 31;
    } else {
        return 0;
    }
}

fn is_correct_date(date: Date) -> bool {
    let (day, month, year) = date;
    (1 <= day && day <= number_of_days_in_month(month, year)) && (1 <= month && month <= 12)
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    loop {
        if cond(start) {
            break start;
        }

        start = fun(start);
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    loop {
        let c = (a + b) / 2.;
        if fun(c).abs() < prec || (b - a) < prec {
            break c;
        }

        if fun(c) * fun(b) < 0. {
            a = c;
        } else {
            b = c;
        }
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.
use std::io;
use rand::{self, Rng};
use std::cmp::Ordering;

fn guessing_game() {
    let secret = rand::rng().random_range(0..=100);
    println!("{secret}");

    let mut last: Option<i32> = None;
    let mut diference: Option<i32> = None;

    loop {
        let mut guess = String::new();        

        io::stdin()
        .read_line(&mut guess)
        .expect("Fatal error!");

        let guess: i32 = match guess.trim().parse() {
            Err(_) => {
                println!("Please, enter a positive number!");
                continue;
            },
            Ok(num) => num
        };

        match last {
            None => last = Some(guess),
            Some(num) => {
                last = Some(guess);
                match diference {
                    None => {
                        diference = Some(guess - num)
                    },
                    Some(dif) => {
                        if (guess - num) != dif {
                            println!("You loose!");
                            break;
                        }
                    }
                }
            }
        }

        match secret.cmp(&guess) {
            Ordering::Greater => println!("Too small!"),
            Ordering::Less => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }        
    }    
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let c_00 = a[0][0] * b[0][0] + a[0][1] * b[1][0];
    let c_01 = a[0][0] * b[0][1] + a[0][1] * b[1][1];
    let c_10 = a[1][0] * b[0][0] + a[1][1] * b[1][0];
    let c_11 = a[1][0] * b[0][1] + a[1][1] * b[1][1];
    [[c_00, c_01], [c_10, c_11]]
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    let mut is_ordered_plus = true;
    let mut is_ordered_minus = true;

    for (i, item) in arr.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if arr[i - 1] > arr[i] {
            is_ordered_plus = false;
        }
    }

    for (i, item) in arr.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if arr[i - 1] < arr[i] {
            is_ordered_minus = false;
        }
    }

    if is_ordered_minus || is_ordered_plus {
        true
    } else {
        false
    }
}

// fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
//     for y in v {
//       if x == y {
//         return true
//       }
//     }
//     return false
// }

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

fn pow(x: u32, n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let half = pow(x, n / 2);
    if n % 2 == 0 {
        half * half
    } else {
        half * half * x
    }
}

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

fn pow_iter(mut x: u32, mut n: u32) -> u32 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res *= x;
        }
        x *= x;
        n /= 2;
    }
    res
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

fn pow_mod(x: u32, n: u32, m: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let half = pow(x, n / 2);
    if n % 2 == 0 {
        (half * half) % m
    } else {
        (half * half * x) % m
    }
}

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {
    for i in 0..arr.len() {
        let mut min = arr[i];
        let mut i_min = i;

        for j in i..arr.len() {
            if arr[j] < min {
                min = arr[j];
                i_min = j;
            }
        }
        let tmp = arr[i];
        arr[i] = min;
        arr[i_min] = tmp;
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: usize) {    
    for i in 0..n {
        let spaces = " ".repeat(n - 1 - i);
        print!("{spaces}");

        let stars = "*".repeat(2 * i + 1);
        print!("{stars}");

        println!();
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A

fn pyramid_abc(mut n: usize) {
    if n > 26 {
        n = 26
    }

    let alphabet = [
        "A", "B", "C", "D", "E", "F",
        "G", "H", "I", "J", "K", "L",
        "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X",
        "Y", "Z"
        ];

    for i in 0..n {
        let spaces = " ".repeat((n - 1) * 2 - 2 * i);
        print!("{spaces}");
        
        let mut row = String::new();
        for j in 0..=i {
            row += alphabet[j];
            row += " ";
        }
        for j in (0..i).rev() {
            row += alphabet[j];
            row += " ";
        }
        print!("{row}");

        println!();
    }
}

fn main() {
    pyramid_abc(10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
    }
}
