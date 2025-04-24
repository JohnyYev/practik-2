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

fn bon_appetit(bill: Vec<usize>, k: usize, b_charged: usize) {
    let total: usize = bill.iter().sum();
    let anna_share = (total - bill[k]) / 2;

    if b_charged == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b_charged - anna_share);
    }
}

fn main() {
    let nk = read_line_as_numbers(); // n and k
    let n = nk[0];
    let k = nk[1];

    let bill = read_line_as_numbers(); // list of item prices
    let b_charged_line = read_line_as_numbers();
    let b_charged = b_charged_line[0];

    bon_appetit(bill[..n].to_vec(), k, b_charged);
}
