use std::io;

fn counting_valleys(steps: i32, path: &str) -> i32 {
    let mut elevation = 0;
    let mut valleys = 0;

    for step in path.chars() {
        if step == 'U' {
            elevation += 1;
            if elevation == 0 {
                valleys += 1;
            }
        } else if step == 'D' {
            elevation -= 1;
        }
    }

    valleys
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // steps (not used directly)

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    let result = counting_valleys(path.len() as i32, path);
    println!("{}", result);
}
