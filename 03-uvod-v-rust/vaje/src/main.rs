// use std::panic;

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    if n == 0 {
        a0
    } else if n == 1 {
        a1
    } else {
        fib(a1, a0 + a1, n - 1)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn is_leap(year: u32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 && year % 400 != 0 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn is_correct_date(date: Date) -> bool {
    let (day, month, year) = date;

    if month == 0 || month > 12 {
        return false;
    }

    let mut num_of_days = 31;

    if month == 4 || month == 6 || month == 9 || month == 11 {
        num_of_days = 30;
    } else if month == 2 {
        if is_leap(year) {
            num_of_days = 29;
        } else {
            num_of_days = 28;
        }
    }

    if day == 0 || day > num_of_days {
        false
    } else {
        true
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteration(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
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

fn bisection(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    if fun(a) * fun(b) > 0. {
        panic!("The function has the same sign at both ends of the interval");
    }

    loop {
        let c = (a + b) / 2.;
        let val = fun(c);
        if b - a < prec || val.abs() < prec {
            break c;
        }
        if fun(a) * fun(c) < 0. {
            b = c;
        } else {
            a = c;
        }
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

// fn guessing_game() {
//     panic!("Not implemented");
// }

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    if arr.len() <= 1 {
        return true;
    }

    let mut asc = true;
    let mut desc = true;
    for (i, &el) in arr.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let prev = arr[i - 1];
        if prev > el {
            asc = false;
        } else if prev < el {
            desc = false;
        }
        if !(desc || asc) {
            break;
        }
    }
    return asc || desc;
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {
    for i in 0..arr.len() {
        let el = arr[i];
        let mut min_idx = i;
        for j in i..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j
            }
        }
        arr[i] = arr[min_idx];
        arr[min_idx] = el;
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    if n == 0 {
        return;
    }

    let max_stars = (2 * (n - 1)) + 1;
    for i in 1..=n {
        let n_stars = 2 * (i - 1) + 1;
        let n_whitespaces = (max_stars - n_stars) / 2;
        let stars = "*".repeat(n_stars as usize);
        let whitespaces = " ".repeat(n_whitespaces as usize);
        println!("{}{}{}", whitespaces, stars, whitespaces);
    }
}

// /// ------------------------------------------------------------------------------------------------
// /// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
// ///       A
// ///     A B A
// ///   A B C B A
// /// A B C D C B A

fn main() {}
