use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let values = read_line_as_numbers();
    let x1 = values[0];
    let v1 = values[1];
    let x2 = values[2];
    let v2 = values[3];

    if v1 == v2 {
        if x1 == x2 {
            println!("YES");
        } else {
            println!("NO");
        }
    } else {
        let numerator = x2 - x1;
        let denominator = v1 - v2;

        if denominator != 0 && numerator % denominator == 0 && (numerator / denominator) >= 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
