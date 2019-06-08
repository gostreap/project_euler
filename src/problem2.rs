pub fn solve() {
    let mut fibo0 = 1;
    let mut fibo1 = 1;
    let mut tmp;
    let mut sum = 0;
    while fibo1 < 4_000_000 {
        tmp = fibo1;
        fibo1 = fibo1 + fibo0;
        fibo0 = tmp;
        if fibo1 % 2 == 0 {
            sum += fibo1;
        }
    }
    println!(
        "La somme des termes pairs plus petit que 4 millions de la suite de Fibonnaci est {}",
        sum
    );
}
