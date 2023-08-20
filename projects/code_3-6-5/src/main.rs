use std::ops::Add;

fn main() {
    let v = vec![3, 99, 7, 10, 1011, 7932, -32, 7, 55, 0];
    println!("{}", sum(&v));

    let v_1to7 = &v[1..7];
    println!("v[1..7] = {:?}", v_1to7);
    println!("v_1to7[2..5] = {:?}", &v_1to7[2..5]);
}

// 分割統治法で合計値を求める
fn sum<T>(arr: &[T]) -> T
where
    T: Add<Output = T> + Copy,
{
    let len = arr.len();
    if len == 1 {
        arr[0]
    } else {
        let m = len / 2;
        sum(&arr[..m]) + sum(&arr[m..])
    }
}

#[test]
fn test_sum() {
    let v = vec![3, 99, 7, 10, 1011, 7932, -32, 7, 55, 0];
    let sum_exp = v.iter().sum::<i32>(); // 9092

    assert_eq!(sum(&v), sum_exp);
}
