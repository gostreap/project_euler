pub fn solve() {
    let mut result = 1;
    for i in 1..21 {
        result = lcm(result, i);
    }
    println!("Le ppcm des entiers de 1 à 20 est {}", result);
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut n = a;
    let mut m = b;
    while n != m {
        if n > m {
            n = n - m;
        } else {
            m = m - n;
        }
    }
    n
}
