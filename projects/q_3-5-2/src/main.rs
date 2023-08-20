use rand::prelude::*;

const CHUNK_SIZE: usize = 10_000;
const CHUNK_NUMBER: usize = 1_000_0;

fn main() {
    let mut rng = thread_rng();
    let mut count: usize = 0;

    for i in 0..CHUNK_NUMBER {
        let data: [(f64, f64); CHUNK_SIZE] = rng.gen();

        count += data
            .into_iter()
            .filter(|(x, y)| {
                let x = 6.0 * x;
                let y = 9.0 * y;
                let in_c1 = (x - 3.0) * (x - 3.0) + (y - 3.0) * (y - 3.0) <= 9.0;
                let in_c2 = (x - 3.0) * (x - 3.0) + (y - 7.0) * (y - 7.0) <= 4.0;
                in_c1 || in_c2
            })
            .count();

        println!("{} %", 100.0 * i as f64 / CHUNK_NUMBER as f64);
    }

    println!("Number of points = {}", count);
    println!(
        "Estimated S = {}",
        54.0 * (count) as f64 / (CHUNK_SIZE * CHUNK_NUMBER) as f64
    );
}
