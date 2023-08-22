use std::{
    fmt::Debug,
    io,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

fn main() {
    // 入力読み込み
    let a: Vec<f64> = read_line("Error at reading a");
    let b: Vec<f64> = read_line("Error at reading b");
    let c: Vec<f64> = read_line("Error at reading c");
    let a: Point<f64> = Point { x: a[0], y: a[1] };
    let b: Point<f64> = Point { x: b[0], y: b[1] };
    let c: Point<f64> = Point { x: c[0], y: c[1] };

    let ba: Point<f64> = b.clone() - a.clone();
    let bc: Point<f64> = b.clone() - c.clone();
    let ac: Point<f64> = a.clone() - c.clone();

    let distance = if ba.inner_product(&bc) > 0_f64 && bc.inner_product(&ac) > 0_f64 {
        // B->C ベクトルの長さ
        let bc_len: f64 = bc.length_2().sqrt();

        // // B->C に平行な単位ベクトル
        // let dir_bc: Point<f64> = bc.div(bc_len);

        // // B->C に垂直な単位ベクトル
        // let ver_bc: Point<f64> = Point {
        //     x: -dir_bc.y,
        //     y: dir_bc.x,
        // };

        // // B->A の B->C に垂直な成分
        // ba.inner_product(&ver_bc).abs()

        // (B->A と B->C の外積) = BA の長さ x BC の長さ x sin∠ABC
        let op = ba.outer_product(&bc).abs();

        // BA の長さ x sin∠ABC
        op / bc_len
    } else if ba.inner_product(&bc) <= 0_f64 {
        ba.length_2().sqrt()
    } else {
        ac.length_2().sqrt()
    };

    println!("{}", distance);
}

// 標準入力をベクタに取り込むための関数
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

#[derive(Clone)]
struct Point<T> {
    x: T,
    y: T,
}

// ベクトルの和の実装
impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Point<T>) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// ベクトルの差の実装
impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// ベクトルの定数倍の実装
impl<T> Point<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul(&self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// ベクトルの定数分の１の実装
impl<T> Point<T>
where
    T: Div<Output = T> + Copy,
{
    fn div(&self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// 内積の実装
impl<T> Point<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    fn inner_product(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    fn length_2(&self) -> T {
        self.inner_product(self)
    }
}

// 外積の実装
impl<T> Point<T>
where T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    fn outer_product(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}
