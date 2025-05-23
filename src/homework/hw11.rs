fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..n {
        let val = ((i * 37 + 11) % 90) as i32 + 10;
        v.push(val);
    }
    v
}

fn min_adjacent_sum(data: &[i32]) -> Option<(usize, usize, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_idx = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx = i;
        }
    }

    Some((min_idx, min_idx + 1, min_sum))
}

fn main() {
    let data = gen_random_vector(20);

    println!("indexes: {}", (0..data.len()).map(|i| format!("{:2}", i)).collect::<Vec<_>>().join("  "));
    println!("data:    {:?}", data);

    if let Some((i1, i2, sum)) = min_adjacent_sum(&data) {
        println!("indexes: {}\\__ __/", "    ".repeat(i1));
        println!("min adjacent sum={}+{}={} at indexes:{},{}", data[i1], data[i2], sum, i1, i2);
    } else {
        println!("Not enough data for adjacent pairs");
    }
}
