use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let n: Vec<f32> = read_line("Error at reading N");
    let n: f32 = n[0];
    let mut questions: Vec<(u8, u8)> = Vec::new();
    for _ in 0..(n as usize) {
        let input = read_line("Error at reading P and Q");
        let p = input[0];
        let q = input[1];
        questions.push((p, q));
    }

    let output: f32 = questions.into_iter().map(|(p, q)| q as f32 / (p as f32)).sum();

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
