use std::io;

fn time_conversion(s: &str) -> String {
    let mut hour: i32 = s[..2].parse().unwrap();
    let is_pm = s.contains("PM");

    if is_pm {
        if hour != 12 {
            hour += 12;
        }
    } else {
        if hour == 12 {
            hour = 0;
        }
    }

    format!("{:02}:{:02}:{:02}", hour, &s[3..5].parse::<i32>().unwrap(), &s[6..8].parse::<i32>().unwrap())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("{}", time_conversion(input));
}
