const WIDTH: usize = 30;
const HEIGHT: usize = 15;

pub fn draw_envelope() {
    let mut result = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ch = if y == 0 || y == HEIGHT - 1 {
                '*'
            } else if x == 0 || x == WIDTH - 1 {
                '*'
            } else if x == y * 2 && x < WIDTH / 2 {
                '*'
            } else if x == WIDTH - 1 - y * 2 && x > WIDTH / 2 {
                '*'
            } else if x == WIDTH / 2 - y && x >= 0 && y >= HEIGHT / 2 {
                '*'
            } else if x == WIDTH / 2 + y - HEIGHT + 1 && y >= HEIGHT / 2 {
                '*'
            } else {
                ' '
            };

            result.push(ch);
        }
        result.push('\n');
    }

    print!("{}", result);
}
