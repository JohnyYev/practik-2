fn main() {
    const W: u32 = 21;
    const H: u32 = 11;

    for y in 0..H {
        for x in 0..W {
            let mid = W / 2;
            let top_half = y <= H / 2;
            let offset = if top_half { y } else { H - 1 - y };

            let left = mid - offset;
            let right = mid + offset;

            let to_show = x >= left && x <= right;
            let sym = if to_show { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}
