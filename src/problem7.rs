pub fn solve() {
    let mut primes = Vec::with_capacity(10001);
    primes.push(2);
    let mut n = 3;
    let mut prime;
    while primes.len() != 10001 {
        prime = true;
        for i in primes.iter() {
            if n % i == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.push(n);
        }
        n += 2;
    }
    println!("Le 10001ème nombre premier est {}", primes[10000])
}
