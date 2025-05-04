fn page_count(n: i32, p: i32) -> i32 {
    let front = p / 2;
    let back = (n / 2) - (p / 2);
    front.min(back)
}

fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = lines.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = page_count(n, p);
    println!("{}", result);
}
