fn draw_tree(levels: usize) {
    let total_lines = (1..=levels).sum::<usize>();
    let max_width = 2 * total_lines - 1;

    for level in 1..=levels {
        println!("{:^width$}", "*", width = max_width);

        for line_in_level in 1..=level {
            let stars_count = 2 * line_in_level - 1;
            let stars = "*".repeat(stars_count);
            println!("{:^width$}", stars, width = max_width);
        }
    }
}

fn main() {
    let levels = 5;
    draw_tree(levels);
}
