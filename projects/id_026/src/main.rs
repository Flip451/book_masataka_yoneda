use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let n: Vec<u128> = read_line("Error at reading N");
    let n: f64 = n[0] as f64;

    let mut output: f64 = 1.0;
    for i in 1..(n as usize) {
        output += n as f64/ (n as f64 - i as f64);
    }

    println!("{:.9}", output);
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
