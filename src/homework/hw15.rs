fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    for &m in digits.iter() {
        for &u in digits.iter() {
            if u == m { continue; }
            for &x in digits.iter() {
                if x == m || x == u { continue; }
                for &a in digits.iter() {
                    if [m, u, x].contains(&a) { continue; }
                    for &s in digits.iter() {
                        if [m, u, x, a].contains(&s) { continue; }
                        for &l in digits.iter() {
                            if [m, u, x, a, s].contains(&l) { continue; }
                            for &o in digits.iter() {
                                if [m, u, x, a, s, l].contains(&o) { continue; }
                                for &n in digits.iter() {
                                    if [m, u, x, a, s, l, o].contains(&n) { continue; }

                                    let muha = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    if muha * a == slon {
                                        println!("  {}\nx    {}\n------\n  {}\n", muha, a, slon);
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Загальна кількість рішень: {}", count);
}