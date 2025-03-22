fn staircase(n: usize) {
    for i in 1..=n {
        println!("{:>width$}", "#".repeat(i), width = n);
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    staircase(n);
}
