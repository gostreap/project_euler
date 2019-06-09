pub fn solve() {
    let mut triangle_num = 1;
    let mut index = 1;
    while count_divisors(triangle_num) < 500 {
        index += 1;
        triangle_num += index;
    }
    println!(
        "Le premier nombre triangulaire ayant au moins 500 diviseurs est {}",
        triangle_num
    );
}

fn count_divisors(n: u64) -> u32 {
    let mut count = 0;
    let mut i = 1;
    while i * i < n {
        if n % i == 0 {
            count += 2
        }
        i += 1;
    }
    if i * i == n {
        count += 1;
    }
    count
}
