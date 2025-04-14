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

fn breaking_records(scores: Vec<i32>) -> (i32, i32) {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    (max_count, min_count)
}

fn main() {
    let _ = read_line_as_numbers(); // not used: number of scores
    let scores = read_line_as_numbers();

    let (max, min) = breaking_records(scores);
    println!("{} {}", max, min);
}
