fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len();
    if n == 0 {
        return Some(0);
    }
    let total: u32 = shipments.iter().sum();
    if total as usize % n != 0 {
        return None;
    }
    let avg = total / n as u32;
    let mut moves = 0;
    for &load in shipments {
        if load > avg {
            moves += (load - avg) as usize;
        }
    }
    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    let mut shipments = Vec::new();
    for i in 0..n-1 {
        shipments.push(((i * 7 + 13) % 10 + 1) as u32);
    }
    let sum: u32 = shipments.iter().sum();
    let remainder = sum % n as u32;
    let last = if remainder == 0 { 0 } else { n as u32 - remainder };
    shipments.push(last);
    shipments
}

fn main() {
    let s1 = vec![8, 2, 2, 4, 4];
    println!("{:?}", count_permutation(&s1));

    let s2 = vec![9, 3, 7, 2, 9];
    println!("{:?}", count_permutation(&s2));

    let s3 = gen_shipments(10);
    println!("{:?}", s3);
    println!("{:?}", count_permutation(&s3));
}
