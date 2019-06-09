pub fn solve() {
    let mut n = 9;
    loop {
        if !is_goldbach(n) {
            break;
        } else {
            n += 2;
        }
    }
    println!("Le plus petit entier impair composite ne pouvant pas s'écrire comme la somme d'un nombre premier est du double d'un carré est {}",n);
}

fn is_goldbach(n: u64) -> bool {
    if !is_prime(n) {
        let mut i = 1;
        let mut double_squared = 2;
        while double_squared < n {
            if is_prime(n - double_squared) {
                return true;
            }
            i += 1;
            double_squared = 2 * i * i;
        }
        return false;
    } else {
        return true;
    }
}

// https://en.wikipedia.org/wiki/Primality_test#Simple_methods
fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i < n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
