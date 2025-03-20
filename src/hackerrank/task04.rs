fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        primary_diagonal += arr[i][i];
        secondary_diagonal += arr[i][n - 1 - i];
    }

    (primary_diagonal - secondary_diagonal).abs()
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut arr = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        arr.push(row);
    }

    println!("{}", diagonal_difference(arr));
}
