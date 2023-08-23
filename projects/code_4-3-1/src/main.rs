const N: usize = 10;
const X_S: f64 = 2_f64;

fn main() {
    let mut x = X_S;
    for _ in 0..N {
        let new_x = x * 2_f64 / 3_f64 + 2_f64 / x / x / 3_f64;
        // let new_x = x_last / 2_f64 + 1_f64 / x_last;
        x = new_x;
        println!("{:.36}", x);
    }
}
