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

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}

fn gcd_of_list(nums: &[i32]) -> i32 {
    nums.iter().copied().reduce(|a, b| gcd(a, b)).unwrap()
}

fn lcm_of_list(nums: &[i32]) -> i32 {
    nums.iter().copied().reduce(|a, b| lcm(a, b)).unwrap()
}

fn main() {
    let _ = read_line_as_numbers(); // sizes of a and b (not used)
    let a = read_line_as_numbers();
    let b = read_line_as_numbers();

    let lcm_a = lcm_of_list(&a);
    let gcd_b = gcd_of_list(&b);

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    println!("{}", count);
}
