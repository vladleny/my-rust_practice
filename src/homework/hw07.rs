fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn main() {
    let examples = ["Hello", "Привет"];
    for &word in &examples {
        let inverted = invert_the_case(word.to_string());
        println!("{} -> {}", word, inverted);
    }
}
