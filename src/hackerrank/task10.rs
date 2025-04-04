use std::io;

fn round_grade(grade: i32) -> i32 {
    if grade < 38 {
        return grade;
    }
    let next_multiple = ((grade / 5) + 1) * 5;
    if next_multiple - grade < 3 {
        return next_multiple;
    }
    grade
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut grades = Vec::new();
    for _ in 0..n {
        let mut grade_input = String::new();
        io::stdin().read_line(&mut grade_input).unwrap();
        let grade: i32 = grade_input.trim().parse().unwrap();
        grades.push(round_grade(grade));
    }

    for grade in grades {
        println!("{}", grade);
    }
}
