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
            .filter(|(x, y)| x * x + y * y <= 1.0)
            .count();

        println!("{} %", 100.0 * i as f64 / CHUNK_NUMBER as f64);
    }

    println!(
        "Estimated pi = {}",
        (4 * count) as f64 / (CHUNK_SIZE * CHUNK_NUMBER) as f64
    );
}
