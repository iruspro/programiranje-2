// use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

// fn fib(a0: u32, a1: u32, n: u32) -> u32 {
//     if n == 0 {
//         return a0;
//     } else if n == 1 {
//         return a1;
//     } 
//     return fib(a1, a0 + a1, n - 1);    
// }

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

// fn is_leap(year: u32) -> bool {
//     if (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0) {
//         return true;
//     } else {
//         return false;
//     }
// }

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto

// type Date = (u32, u32, u32);

// fn get_number_of_days_in(month: u32, year: u32) -> u32 {
//     let mut number_of_days = 0;

//     if month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12 {
//         number_of_days = 31;
//     } else if month == 4 || month == 6 || month == 9 || month == 11 {
//         number_of_days = 30;
//     } else if month == 2 {
//         if is_leap(year) {
//             number_of_days = 29;
//         } else {
//             number_of_days = 28;
//         }
//     } 
    
//     return number_of_days
// }

// fn is_correct_date(date: Date) -> bool {
//     let (day, month, year) = date;
//     return (1 <= day) && (day <= get_number_of_days_in(month, year)) && (1 <= month) && (month <= 12)
// }

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

// fn iteraction(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
//     loop {
//         if cond(start) {
//             break;
//         }
//         start = fun(start);
//     }
//     return start;
// }

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

// fn abs(x: f64) -> f64 {
//     if x >= 0. {
//         return x;
//     } else {
//         return -x;
//     }
// }

// fn bisection(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
//     let mut middle = (a + b) / 2.;
//     while abs(fun(middle)) >= prec && b - a >= prec {
//         if fun(a) * fun(middle) < 0. {
//             b = middle;
//             middle = (a + middle) / 2.;
//         } else {
//             a = middle;
//             middle = (middle + b) / 2.;
//         }
//     }
//     middle    
// }

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

// fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
//     let a11 = a[0][0] * b[0][0] + a[0][1] * b[1][0];
//     let a12 = a[0][0] * b[0][1] + a[0][1] * b[1][1];
//     let a21 = a[1][0] * b[0][0] + a[1][1] * b[1][0];
//     let a22 = a[1][0] * b[0][1] + a[1][1] * b[1][1];
//     [[a11, a12], [a21, a22]]
// }

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

// fn ordered(arr: &[u32]) -> bool {
//     let arr_len = arr.len();
//     let mut i = 0;
//     while i + 1 < arr_len {
//         if arr[i] > arr[i + 1] {
//             return false
//         };
//         i = i + 1;
//     };
//     true
// }

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

// fn pow(mut x: u32, mut n: u32) -> u32 {
//     if n == 0 {
//         return 1
//     }

//     if n % 2 == 0 {
//         pow(x * x, n / 2)
//     } else {
//         x * pow(x * x, n / 2)
//     }
// }

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

// fn pow_iter(mut x: u32, mut n: u32) -> u32 {
//     let mut result = 1;
//     while n != 0 {
//         if n % 2 == 1 {
//             result = result * x;
//             n = n - 1;
//         }
//         x = x * x;
//         n = n / 2;
//     }
//     result
// }

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

// fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32 {
//     if n == 0 {
//         return x % m
//     }

//     if n % 2 == 0 {
//         pow_mod(x * x, n / 2, m) % m
//     } else {
//         (x * pow_mod(x * x, n / 2, m)) % m
//     }
// }


/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

// fn selection_sort(arr: &mut [u32]) {
//     let length = arr.len();
//     let mut pos = 0;
    
//     while pos < length {
//         let mut min = arr[pos];
//         let mut min_ind = pos;
//         let mut i = min_ind;
        
//         while i < length {
//             let elem = arr[i];
//             if elem < min {
//                 min = elem;
//                 min_ind = i;
//             }
//             i += 1;
//         }
        
//         if min_ind != pos {
//             let tmp = arr[pos];
//             arr[pos] = min;
//             arr[min_ind] = tmp;
//         }
        
//         pos += 1;
//     }
// }

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn mult_print(ch: char, n: u32) {
    for _ in 0..n {
        print!("{}", ch)
    }
}

// fn pyramid(n: u32) {
//     for i in 0..n {  
//         mult_print(' ', n - (i + 1));     
//         mult_print('*', i *2 + 1);
//         print!("\n");
//     }
// }

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahko predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A

fn abc_pyramid(n: u32) {
    let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    // let alphabet: Vec<char> = alphabet.chars().collect();

    mult_print(' ', 2 * n + 1);
    println!("A");
    
    for i in 0..n {
        mult_print(' ', n - (i + 1) + (n - i));
        
        let mut k = 0;
        while k <= (i * 2 + 1) / 2 {
            let ch = match alphabet.chars().nth(k as usize) {
                Some (ch) => ch,
                _ => '*',
            };
            print!{"{} ", ch};
            k += 1;
        } 

        loop {
            let ch = match alphabet.chars().nth(k as usize) {
                Some (ch) => ch,
                _ => '*',
            };
            print!{"{} ", ch};
            if k == 0 {
                break
            } else {
                k -= 1
            }
        }
        println!{""};
    }
}

fn main() {
    abc_pyramid(30);
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

    #[test]
    fn test_ordered() {
        let result = ordered(&[]);
        assert_eq!(result, true);
        let result = ordered(&[1]);
        assert_eq!(result, true);
        let result = ordered(&[1, 1, 2, 3, 4, 5]);
        assert_eq!(result, true);
        let result = ordered(&[2, 1]);
        assert_eq!(result, false);
    }
}
