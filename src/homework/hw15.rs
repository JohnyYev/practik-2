fn main() {

    let mut solutions = Vec::new();


    let digits = [1,2,3,4,5,6,7,8];


    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x==m||x==u { continue; }
                for &a in &digits {
                    if a==m||a==u||a==x { continue; }

                    let muha = 1000*m + 100*u + 10*x + a;
                    let prod = muha * a;

                    if prod < 1000 || prod > 9999 { continue; }
                    let s = prod/1000;
                    let l = (prod/100)%10;
                    let o = (prod/10)%10;
                    let n = prod%10;

                    if [s,l,o,n].iter().any(|&d|
                        d==m||d==u||d==x||d==a) { continue; }

                    if s==l||s==o||s==n||l==o||l==n||o==n { continue; }

                    solutions.push((m,u,x,a,s,l,o,n, muha, prod));
                }
            }
        }
    }


    for (m,u,x,a,s,l,o,n, muha, prod) in &solutions {

        println!("  {}{}{}{}", m, u, x, a);

        print!("x");
        let width = 4;
        let pad = width - 1;
        for _ in 0..pad { print!(" "); }
        println!("{}", a);

        let line_len = width + 2;
        print!("  ");
        for _ in 0..line_len { print!("-"); }
        println!();

        let res_pad = line_len - width;
        print!("  ");
        for _ in 0..res_pad { print!(" "); }
        println!("{}{}{}{}", s, l, o, n);
        println!();
    }

    println!("Total solutions: {}", solutions.len());
}
