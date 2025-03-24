fn mini_max_sum(arr: &[u64]) {
    let total: u64 = arr.iter().sum();
    let min_sum = total - arr.iter().max().unwrap();
    let max_sum = total - arr.iter().min().unwrap();
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    mini_max_sum(&arr);
}
