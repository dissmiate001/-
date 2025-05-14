// src/homework/hw04.rs

const HEIGHT: usize = 9; // Має бути непарне число для симетричного ромба
const WIDTH: usize = HEIGHT; // Для ромба краще мати квадратне полотно

pub fn draw_diamond() {
    let mut output = String::new();

    let mid = HEIGHT / 2;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y <= mid {
                if x == mid - y || x == mid + y {
                    output.push('*');
                } else {
                    output.push(' ');
                }
            } else {
                if x == y - mid || x == (WIDTH - 1) - (y - mid) {
                    output.push('*');
                } else {
                    output.push(' ');
                }
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
