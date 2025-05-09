fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let n = ((n % len as isize) + len as isize) % len as isize;
    let n = n as usize;

    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (n, expected) in shifts.iter() {
        let result = rotate(s.clone(), *n);
        println!(
            "rotate({}, {}) = {} | {}",
            s, n, result,
            if result == *expected { "OK" } else { "FAIL" }
        );
    }
}
