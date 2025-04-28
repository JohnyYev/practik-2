use std::io;
use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn sock_merchant(socks: Vec<u32>) -> u32 {
    let mut counts = HashMap::new();
    for sock in socks {
        *counts.entry(sock).or_insert(0) += 1;
    }

    counts.values().map(|&count| count / 2).sum()
}

fn main() {
    let _ = read_line_as_numbers(); // number of socks (not used directly)
    let socks = read_line_as_numbers();
    let result = sock_merchant(socks);
    println!("{}", result);
}
