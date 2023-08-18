// use core::fmt::Debug;
// use num_traits::Num;
// use std::{io, str::FromStr};
use std::io;

fn main() -> io::Result<()> {
    // let _: Vec<u64> = read_line("Error at reading N");
    // let a: Vec<u8> = read_line("Error at reading A_1, ..., A_N");
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let _: usize = input.trim().parse().expect("n is incorrect.");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let a: Vec<&str> = input.trim().split_whitespace().collect();

    let mut color: [u128; 3] = [0, 0, 0];

    for a_i in a.into_iter() {
        let a_i: usize = a_i.parse().expect("A_i is incorrect.");
        color[a_i - 1] += 1;
    }

    let output: u128 = color.iter().map(|c: &u128| combination_two(*c)).sum();

    println!("{}", output);

    Ok(())
}

fn combination_two(a: u128) -> u128 {
    if a < 2 {
        return 0;
    } else {
        a * (a - 1) / 2
    }
}

// fn combination(a: u128, b: u128) -> u128 {
//     if a < b {
//         return 0;
//     } else {
//         factorial(a) / factorial(a - b) / factorial(b)
//     }
// }

// fn factorial(a: u128) -> u128 {
//     if a == 0 {
//         return 1;
//     } else {
//         a * factorial(a - 1)
//     }
// }

// fn conbination<T>(a: T, b: T) -> T
// where T: Num + std::cmp::PartialOrd + Copy
// {
//     if a < b {
//         return T::zero();
//     } else {
//         factorial(a) / factorial(a - b) / factorial(b)
//     }
// }

// fn factorial<T>(a: T) -> T
// where T: Num + std::cmp::PartialOrd + Copy
// {
//     if a <= T::zero() {
//         return T::one();
//     } else {
//         a * factorial(a - T::one())
//     }
// }

// fn read_line<T>(err: &str) -> Vec<T>
// where
//     T: FromStr,
//     <T as FromStr>::Err: Debug,
// {
//     let mut input: String = String::new();
//     io::stdin().read_line(&mut input).expect(err);

//     let input: Vec<&str> = input.trim().split_whitespace().collect();
//     let mut elements: Vec<T> = Vec::new();

//     for e in input.into_iter() {
//         let new_elemnt: T = e.trim().parse::<T>().expect(err);
//         elements.push(new_elemnt);
//     }

//     elements
// }
