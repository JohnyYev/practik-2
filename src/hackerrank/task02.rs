use std::io;

fn compare_triplets(a: &[i32], b: &[i32]) -> (i32, i32) {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    (alice_score, bob_score)
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let alice: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let bob: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (alice_score, bob_score) = compare_triplets(&alice, &bob);

    println!("{} {}", alice_score, bob_score);
}
