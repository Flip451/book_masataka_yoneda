fn main() {
    let mut sum: f64 = 0_f64;
    let mut i: i128 = 1;
    while sum < 30_f64 {
        sum += 1_f64 / (i as f64);
        i += 1;
        println!("{}", i);
    }
}
