use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn birthday(s: Vec<i32>, d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=s.len() - m as usize {
        let sum: i32 = s[i..i + m as usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }
    count
}

fn main() {
    let _ = read_line_as_numbers(); // size of the array (not used)
    let s = read_line_as_numbers();
    let dm = read_line_as_numbers();
    let d = dm[0];
    let m = dm[1];

    let result = birthday(s, d, m);
    println!("{}", result);
}
