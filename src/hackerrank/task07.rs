use std::io::{self, BufRead};

fn miniMaxSum(arr: &[i32]) {
    let arr: Vec<i64> = arr.iter().map(|&x| x as i64).collect();
    let total_sum: i64 = arr.iter().sum();

    let min_sum = total_sum - arr.iter().max().unwrap();
    let max_sum = total_sum - arr.iter().min().unwrap();

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
