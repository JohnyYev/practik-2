const WIDTH: usize = 10;  // Ширина конверта
const HEIGHT: usize = 6; // Висота конверта

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 {
                output.push('-'); // Верхня і нижня межа
            } else if x == 0 || x == WIDTH - 1 {
                output.push('|'); // Бічні межі
            } else if x == y || x == WIDTH - y - 1 {
                output.push('/'); // Діагональні лінії
            } else {
                output.push(' '); // Порожній простір всередині
            }
        }
        output.push('\n');
    }

    print!("{}", output); // Вивід конверта
}
