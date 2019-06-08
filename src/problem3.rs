use std::collections::LinkedList;

const VALUE: u64 = 600851475143;

pub fn solve() {
    let prime_factors = trial_division(VALUE);
    print!("{} = ", VALUE);
    for i in prime_factors.iter() {
        print!("{}*", i);
    }
    println!("1");
    let res = match prime_factors.back() {
        Some(x) => *x,
        None => 0,
    };
    println!("Le plus grand diviseur premier de {} est {}", VALUE, res);
}

// https://en.wikipedia.org/wiki/Trial_division
fn trial_division(n: u64) -> LinkedList<u64> {
    let mut prime_factors: LinkedList<u64> = LinkedList::new();
    let mut tmp = n;
    while tmp % 2 == 0 {
        prime_factors.push_back(2);
        tmp /= 2;
    }
    let mut f = 3;
    while f * f < tmp {
        if tmp % f == 0 {
            prime_factors.push_back(f);
            tmp /= f;
        } else {
            f += 2;
        }
    }
    if tmp != 1 {
        prime_factors.push_back(tmp);
    }
    prime_factors
}
