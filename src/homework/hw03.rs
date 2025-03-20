fn main() {
    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let is_hor: bool = y == 0 || y == H - 1;
            let is_ver: bool = x == 0 || x == W - 1;
            let is_diag: bool = x == y;
            let is_diag2: bool = y == W - 1 - x;
            let to_show: bool = is_hor || is_ver || is_diag || is_diag2;

            let sym: char = if to_show { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}
