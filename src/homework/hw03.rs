#[test]
fn main() {
    const W: u32 = 29;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let is_hor: bool = y == 0 || y == H - 1;
            let is_ver: bool = x == 0 || x == W - 1;
            let is_diag1: bool = x == (y * (W - 1)) / (H - 1);
            let is_diag2: bool = x == (W - 1) - (y * (W - 1)) / (H - 1);

            let to_show: bool = is_hor || is_ver || is_diag1 || is_diag2;
            let sym: char = if to_show { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}
