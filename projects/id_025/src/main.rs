use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let n: Vec<usize> = read_line("Error at reading N");
    let _ = n[0];
    let a: Vec<u8> = read_line("Error at reading A");
    let b: Vec<u8> = read_line("Error at reading B");

    let output: f32 = a.into_iter().map(|a| a as f32 / 3.0).sum::<f32>()
        + b.into_iter().map(|b| 2.0 * b as f32 / 3.0).sum::<f32>();

    println!("{:?}", output);
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
