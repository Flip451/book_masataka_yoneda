use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let _: Vec<u128> = read_line("Error at reading N");
    let a: Vec<usize> = read_line("Error at reading N");

    const SIZE: usize = 100000;

    let mut counts: [u128; SIZE - 1] = [0; SIZE - 1];
    for a_i in a.into_iter() {
        counts[a_i - 1] += 1;
    }

    let mut output = 0;
    for i in 1..=(SIZE / 2 - 1) {
        output += counts[i - 1] * counts[SIZE - i - 1];
    }

    output += counts[SIZE / 2 - 1] * (counts[SIZE / 2 - 1] - 1) / 2;

    println!("{}", output);
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
