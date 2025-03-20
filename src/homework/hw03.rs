const WIDTH: usize = 30;
const HEIGHT: usize = 14;

fn main() {
    let mut envelope = vec![vec![' '; WIDTH]; HEIGHT];
    for x in 0..WIDTH {
        envelope[0][x] = '*';
        envelope[HEIGHT - 1][x] = '*';
    }
    for y in 0..HEIGHT {
        envelope[y][0] = '*';
        envelope[y][WIDTH - 1] = '*';
    }
    for y in 0..HEIGHT {
        let left_x = y * WIDTH / HEIGHT;
        let right_x = WIDTH - 1 - left_x;
        envelope[y][left_x] = '*';
        envelope[y][right_x] = '*';
    }
    print!("{}\n", envelope.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
}