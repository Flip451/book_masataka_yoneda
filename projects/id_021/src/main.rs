use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let input: Vec<u128> = read_line("Error at reading N");
    let n = input[0];
    let r = input[1];

    let output: u128 = combination(n, r);

    println!("{}", output);
}

fn combination(a: u128, b: u128) -> u128 {
    let b = if b > a / 2 { a - b } else { b };
    let mut mul = 1;
    let mut div = 1;
    for i in 0..b {
        mul *= a - i;
        div *= i + 1;
    }
    mul / div
}

fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut elements: Vec<T> = Vec::new();

    for e in input.into_iter() {
        let new_elemnt: T = e.trim().parse::<T>().expect(err);
        elements.push(new_elemnt);
    }

    elements
}
