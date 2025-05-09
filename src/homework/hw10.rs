fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    for (n, expected) in data.iter() {
        let result = is_palindrome(*n);
        println!(
            "is_palindrome({}) = {} | {}",
            n, result,
            if result == *expected { "OK" } else { "FAIL" }
        );

        if result != *expected {
            std::process::exit(1);
        }
    }
}
