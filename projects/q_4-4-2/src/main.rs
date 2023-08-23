const N: usize = 100;

fn main() {
    let mut sum = 0_f64;

    for i in 0..N {
        let mut adder = 1_f64;
        for j in 1..=i {
            adder *= (- 4_f64.ln()) / (2_f64 * j as f64 + 1_f64);
        }
        sum += adder;
        println!("adder: {}", adder);
    }

    println!("{}", 2_f64 * sum);
}
