use std::io;

fn a_very_big_sum(arr: Vec<i64>) -> i64 {
    arr.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = a_very_big_sum(arr);
    println!("{}", result);
}
