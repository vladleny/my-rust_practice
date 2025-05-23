fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let count = 1 << n; // 2^n
    let mut result = Vec::with_capacity(count as usize);

    for i in 0..count {
        let s = format!("{:0width$b}", i, width = n as usize);
        result.push(s);
    }

    result
}

fn main() {
    for n in 0..=4 {
        let codes = gray(n);
        println!("n = {}:", n);
        for code in codes {
            println!("{}", code);
        }
        println!("♥♥♥");
    }
}