pub fn solve() {
    let n = 2_000_000;
    let res = sieves_of_eratosthenes(n);
    println!(
        "La somme des nombres premiers inférieurs à {} vaut {}",
        n, res
    );
}

// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn sieves_of_eratosthenes(n: u64) -> u64 {
    let mut primes = Vec::with_capacity((n + 1) as usize); //number from 2 to n

    for _ in 0..n + 1 {
        primes.push(true);
    }

    primes[0] = false;
    primes[1] = false;

    for i in 2..((n as f64).sqrt() as usize) {
        if primes[i] {
            let mut j = i * i;
            while j < (n + 1) as usize {
                primes[j] = false;
                j += i;
            }
        }
    }
    let mut res: u64 = 0;
    for i in 2..((n + 1) as usize) {
        if primes[i] {
            res += i as u64
        }
    }
    res
}
