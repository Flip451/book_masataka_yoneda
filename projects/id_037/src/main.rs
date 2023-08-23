use std::{
    fmt::Debug,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

fn main() {
    // 入力読み込み
    let p1: Vec<i128> = read_line("Error at reading x1, y1");
    let p2: Vec<i128> = read_line("Error at reading x2, y2");
    let p3: Vec<i128> = read_line("Error at reading x3, y3");
    let p4: Vec<i128> = read_line("Error at reading x4, y4");

    // 読み取った座標をベクトル化
    let p1: Point<i128> = Point { x: p1[0], y: p1[1] };
    let p2: Point<i128> = Point { x: p2[0], y: p2[1] };
    let p3: Point<i128> = Point { x: p3[0], y: p3[1] };
    let p4: Point<i128> = Point { x: p4[0], y: p4[1] };

    // 行列を計算
    let a_11 = p1.x - p2.x;
    let a_12 = p4.x - p3.x;
    let a_21 = p1.y - p2.y;
    let a_22 = p4.y - p3.y;

    let det = a_11 * a_22 - a_12 * a_21;

    let output = if det == 0_i128 {
        // 二つの線分が平行なとき
        // 他方の点が線分上にあるかを判定する処理を実装
        if (p2.x - p1.x) * (p3.y - p1.y) == (p2.y - p1.y) * (p3.x - p1.x) {
            // 二つの線分が同一直線上にある時
            if p1.x != p2.x {
                // 線分が x 軸に垂直でない場合
                // 各線分の端点の x 座標の大きさを比べてラベル付け
                let (x1, x2) = if p1.x < p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };
                let (x3, x4) = if p3.x < p4.x {
                    (p3.x, p4.x)
                } else {
                    (p4.x, p3.x)
                };
                // 二つの線分が共通領域を持つかを判定
                if x1 < x3 && x2 < x3 {
                    "No"
                } else if x4 < x1 && x4 < x2 {
                    "No"
                } else {
                    "Yes"
                }
            } else {
                // 線分が x 軸に垂直な場合
                // 各線分の端点の y 座標の大きさを比べてラベル付け
                let (y1, y2) = if p1.y < p2.y {
                    (p1.y, p2.y)
                } else {
                    (p2.y, p1.y)
                };
                let (y3, y4) = if p3.y < p4.y {
                    (p3.y, p4.y)
                } else {
                    (p4.y, p3.y)
                };
                // 二つの線分が共通領域を持つかを判定
                if y1 < y3 && y2 < y3 {
                    "No"
                } else if y4 < y1 && y4 < y2 {
                    "No"
                } else {
                    "Yes"
                }
            }
        } else {
            // 二つの線分が同一直線上にない時
            "No"
        }
    } else {
        // 二つの線分が平行でないとき
        // 逆行列を [(x1 - x3), (y1 - y3)]^T に作用させる
        // let alpha = (a_22 * (p1.x - p3.x) - a_12 * (p1.y - p3.y)) / det;
        // let beta = (-a_21 * (p1.x - p3.x) + a_11 * (p1.y - p3.y)) / det;
        // if 0_i128 <= alpha && alpha <= 1_i128 && 0_i128 <= beta && beta <= 1_i128 {
        //     "Yes"
        // } else {
        //     "No"
        // }
        let alpha = (a_22 * (p1.x - p3.x) - a_12 * (p1.y - p3.y));
        let beta = (-a_21 * (p1.x - p3.x) + a_11 * (p1.y - p3.y));
        if alpha * det < 0_i128 || beta * det < 0_i128 {
            "No"
        } else if alpha.abs() <= det.abs() && beta.abs() <= det.abs() {
            "Yes"
        } else {
            "No"
        }
    };

    println!("{}", output);
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

// // ベクトルの定数倍の実装
// impl<T> Point<T>
// where
//     T: Mul<Output = T> + Copy,
// {
//     fn mul(&self, rhs: T) -> Self {
//         Self {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }

// // ベクトルの定数分の１の実装
// impl<T> Point<T>
// where
//     T: Div<Output = T> + Copy,
// {
//     fn div(&self, rhs: T) -> Self {
//         Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }

// // 内積の実装
// impl<T> Point<T>
// where
//     T: Mul<Output = T> + Add<Output = T> + Copy,
// {
//     fn inner_product(&self, rhs: &Self) -> T {
//         self.x * rhs.x + self.y * rhs.y
//     }

//     fn length_2(&self) -> T {
//         self.inner_product(self)
//     }
// }

// // 外積の実装
// impl<T> Point<T>
// where
//     T: Mul<Output = T> + Sub<Output = T> + Copy,
// {
//     fn outer_product(&self, rhs: &Self) -> T {
//         self.x * rhs.y - self.y * rhs.x
//     }
// }
