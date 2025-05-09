use std::io::{self, BufRead};

fn time_conversion(s: &str) -> String {
    let period = &s[s.len() - 2..]; 
    let time = &s[..s.len() - 2];  
    let mut parts: Vec<&str> = time.split(':').collect();

    let hour: u32 = parts[0].parse().unwrap();

    let new_hour = match period {
        "AM" => {
            if hour == 12 { 0 } else { hour }
        },
        "PM" => {
            if hour == 12 { 12 } else { hour + 12 }
        },
        _ => hour, 
    };

    format!("{:02}:{:}:{:}", new_hour, parts[1], parts[2])
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let result = time_conversion(&input);

    println!("{}", result);
}
