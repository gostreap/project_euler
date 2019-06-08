pub fn solve() {
    let mut max = 0;
    let mut product;
    for i in 1..1000 {
        for j in i..1000 {
            product = i * j;
            if max < product && is_palindrome(product) {
                max = product;
            }
        }
    }
    println!(
        "Le plus grand palindrome produit de deux entiers à trois chiffres est {}",
        max
    );
}

// https://stackoverflow.com/a/199218
fn is_palindrome(n: u32) -> bool {
    let mut tmp = n;
    let mut rev = 0;
    let mut dig;
    while tmp > 0 {
        dig = tmp % 10;
        rev = rev * 10 + dig;
        tmp = tmp / 10;
    }
    n == rev
}
