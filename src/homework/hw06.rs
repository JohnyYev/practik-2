fn main() {
    let layers: usize = 5;
    let max_width = 2 * (layers + layers - 1) + 1;

    for layer in 0..layers {
        for row in 0..=layer {
            let stars = 1 + 2 * row;
            let padding = (max_width - stars) / 2;
            print!("{}{}", " ".repeat(padding), "*".repeat(stars));
            println!();
        }
    }
}
