fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    println!("GCD of 120 and 90 is {}", gcd(120, 90));
}
