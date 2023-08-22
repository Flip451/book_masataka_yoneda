use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    // 入力の読み込み
    let n: Vec<usize> = read_line("Error at reading N");
    let _ = n[0];
    let a: Vec<isize> = read_line("Error at reading A");

    // マージソートを実行
    let sorted_a = merge_sort(&a);

    // ソート済みのベクタを適当なフォーマットの文字列に変換
    let output: Vec<String> = sorted_a.into_iter().map(|w| w.to_string()).collect();
    let output = output.join(" ");

    // 出力
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

// マージを利用したソートの実装
fn merge_sort<T>(v: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let n = v.len();
    if n <= 1 {
        // 配列に一つの要素しか含まれないときはソート済みなのでそのまま返す
        Vec::from(v)
    } else {
        // 配列を二分割
        let m = n / 2;
        let a = &v[..m];
        let b = &v[m..];
        // 各々をマージソート
        let a_sorted = merge_sort(a);
        let b_sorted = merge_sort(b);

        // ソート済みのふたつの配列をマージ
        merge(&a_sorted, &b_sorted)
    }
}

// マージ (二つのソート済み配列を一つのソート済み配列に合併する賢い方法)
fn merge<T>(mut a: &[T], mut b: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let mut c: Vec<T> = Vec::new();

    loop {
        if a.len() == 0 {
            // a が空なら b を c の後方に繋げて操作終了
            c.extend_from_slice(b);
            break;
        } else if b.len() == 0 {
            // b が空なら a を c の後方に繋げて操作終了
            c.extend_from_slice(a);
            break;
        } else {
            // a, b の配列の先頭のうち、小さい方を c に移動する
            if a[0] < b[0] {
                // a の先頭をクローンして c に push
                c.push(a[0].clone());
                // a の先頭を除外
                a = &a[1..];
            } else {
                // b の先頭をクローンして c に push
                c.push(b[0].clone());
                // b の先頭を除外
                b = &b[1..];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;

    #[test]
    fn test_merge() {
        let a = vec![1, 2, 5, 8, 9, 30, 100];
        let b = vec![0, 3, 9, 20, 21, 28, 31, 60];
        let expected = vec![0, 1, 2, 3, 5, 8, 9, 9, 20, 21, 28, 30, 31, 60, 100];

        let c = crate::merge(&a, &b);

        assert_eq!(c, expected)
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![3, 99, 7, 10, 1011, 7932, -32, 7, 55, 0];
        let expected = vec![-32, 0, 3, 7, 7, 10, 55, 99, 1011, 7932];

        assert_eq!(merge_sort(&v), expected);
    }
}


// fn merge<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
// where
//     T: PartialOrd
// {
//     let mut c: Vec<T> = Vec::new();

//     a.reverse();
//     b.reverse();

//     loop {
//         if a.len() == 0 {
//             b.reverse();
//             c.extend(b);
//             break;
//         } else if b.len() == 0 {
//             a.reverse();
//             c.extend(a);
//             break;
//         } else {
//             let a_min = a.pop().unwrap();
//             let b_min = b.pop().unwrap();
//             if a_min < b_min {
//                 c.push(a_min);
//                 b.push(b_min);
//             } else {
//                 c.push(b_min);
//                 a.push(a_min);
//             }
//         }
//     }

//     c
// }
