const WIDTH: usize = 11;
const HEIGHT: usize = 11;

fn main() {
    let mut canvas = vec![vec![' '; WIDTH]; HEIGHT];

    let mid = HEIGHT / 2;

    for y in 0..HEIGHT {
        let distance_from_center = if y <= mid {
            mid - y
        } else {
            y - mid
        };
        let stars = WIDTH - 2 * distance_from_center;
        let start_x = distance_from_center;

        for x in 0..stars {
            canvas[y][start_x + x] = '*';
        }
    }

    let output = canvas
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output);
}
