use std::collections::BTreeSet;

const N: u32 = 28123;

pub fn solve() {
    let mut res = 0;
    let abundants = set_of_abundant_number(N);
    let sums = set_of_sum_of_two_abundants(N, &abundants);
    for n in 0..N {
        if !sums.contains(&n) {
            res += n;
        }
    }
    println!("the sum of all the positive integers which cannot be written as the sum of two abundant numbers is {}", res);
}

fn set_of_sum_of_two_abundants(n: u32, abundants: &BTreeSet<u32>) -> BTreeSet<u32> {
    let mut sums = BTreeSet::new();
    let mut sum;
    for a in abundants.iter() {
        if *a > n {
            break;
        }
        for b in abundants.iter() {
            sum = *a + *b;
            if sum > n {
                break;
            }
            sums.insert(sum);
        }
    }
    sums
}

fn set_of_abundant_number(n: u32) -> BTreeSet<u32> {
    let mut abundants = BTreeSet::new();
    for i in 1..n {
        if sum_divisors(i) > i {
            abundants.insert(i);
        }
    }
    abundants
}

fn sum_divisors(n: u32) -> u32 {
    let mut sum = 1;
    let mut i = 2;
    while i * i < n {
        if n % i == 0 {
            sum += i + n / i;
        }
        i += 1;
    }
    if i * i == n {
        sum += i;
    }
    sum
}