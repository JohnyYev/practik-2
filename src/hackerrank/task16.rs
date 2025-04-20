use std::io;

fn read_line_as_numbers() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn divisible_sum_pairs(k: usize, ar: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..ar.len() {
        for j in i + 1..ar.len() {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let nk = read_line_as_numbers(); // n and k
    let n = nk[0];
    let k = nk[1];

    let ar = read_line_as_numbers(); // array of n elements

    let result = divisible_sum_pairs(k, &ar[..n]);
    println!("{}", result);
}
