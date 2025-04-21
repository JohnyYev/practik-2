fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6]; // index 0 is unused

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result = i as i32;
        }
    }

    result
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = migratory_birds(&arr);
    println!("{}", result);
}
