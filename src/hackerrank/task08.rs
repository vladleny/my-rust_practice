use std::io::{self, BufRead};

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&&h| h == *max_height).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let _n = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let candles: Vec<i32> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);
    println!("{}", result);
}
