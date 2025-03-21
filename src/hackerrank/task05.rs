fn plus_minus(arr: Vec<i32>) {
    let n = arr.len() as f64;
    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for &num in &arr {
        if num > 0 {
            positive += 1;
        } else if num < 0 {
            negative += 1;
        } else {
            zero += 1;
        }
    }

    println!("{:.6}", positive as f64 / n);
    println!("{:.6}", negative as f64 / n);
    println!("{:.6}", zero as f64 / n);
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    plus_minus(arr);
}
