/// Day 4: Secure Container
/// 
/// Crack the password to a Venus fuel container

fn generate_passwords() -> Vec<usize> {
    let mut results: Vec<usize> = Vec::new();

    for a in 1..=6 {
        for b in 0..=9 {
            for c in 0..=9 {
                for d in 0..=9 {
                    for e in 0..=9 {
                        for f in 0..=9 {
                            let num = a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f;
                            if num < 134792 || num > 675810 {
                                continue;
                            }

                            if a != b && b != c && c != d && d != e && e != f {
                                continue;
                            }

                            if a > b || b > c || c > d || d > e || e > f {
                                continue;
                            }

                            results.push(num);
                        }
                    }
                }
            }
        }
    }

    results
}

fn generate_passwords2() -> Vec<usize> {
    let mut results: Vec<usize> = Vec::new();

    for a in 1..=6 {
        for b in 0..=9 {
            for c in 0..=9 {
                for d in 0..=9 {
                    for e in 0..=9 {
                        for f in 0..=9 {
                            let num = a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f;
                            if num < 134792 || num > 675810 {
                                continue;
                            }

                            if !((a == b && b != c) ||
                                 (b == c && a != b && c != d) ||
                                 (c == d && b != c && d != e) ||
                                 (d == e && c != d && e != f) ||
                                 (e == f && d != e)) {
                                continue;
                            }

                            if a > b || b > c || c > d || d > e || e > f {
                                continue;
                            }

                            results.push(num);
                        }
                    }
                }
            }
        }
    }

    results
}

pub fn run() {
    let candidates = generate_passwords();
    println!("There are {} possible passwords.", candidates.len());
    let candidates2 = generate_passwords2();
    println!("For round 2, there are {} possible passwords.", candidates2.len());
}