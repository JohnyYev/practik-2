use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1;
    }

    let average = total / n;
    let mut moves = 0;
    let mut cumulative_diff = 0i32;

    for &weight in shipments.iter() {
        let diff = weight as i32 - average as i32;
        cumulative_diff += diff;
        moves += cumulative_diff.abs();
    }

    moves
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let average = rng.gen_range(5..=20);
    let mut shipments = vec![average; n];

    for _ in 0..n / 2 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 1 {
            shipments[i] -= 1;
            shipments[j] += 1;
        }
    }

    shipments
}

fn print_shipments(shipments: &[u32]) {
    println!("Shipments: {:?}", shipments);
    let moves = count_permutation(&shipments.to_vec());

    if moves == -1 {
        println!("Розподіл неможливий (сума не ділиться на кількість кораблів)");
    } else {
        println!("Мінімальна кількість переміщень: {moves}");
    }
}
fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];
    let example3 = vec![1, 1, 1, 6];

    println!("--- Приклад 1 ---");
    print_shipments(&example1);
    println!();

    println!("--- Приклад 2 ---");
    print_shipments(&example2);
    println!();

    println!("--- Приклад 3 ---");
    print_shipments(&example3);
    println!();

    println!("--- Згенерований розподіл ---");
    let generated = gen_shipments(10);
    print_shipments(&generated);
}
