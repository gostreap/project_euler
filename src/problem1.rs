pub fn solve() {
    let mut res = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            res += i;
        }
    }
    println!(
        "La somme des multiples de 3 ou 5 en dessous de 1000 est {}",
        res
    );
}
