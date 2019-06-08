pub fn solve() {
    let mut sum_of_squared = 0;
    let mut squared_of_sum = 0;
    for i in 1..101 {
        sum_of_squared += i * i;
        squared_of_sum += i;
    }
    squared_of_sum *= squared_of_sum;
    let res = squared_of_sum - sum_of_squared;
    println!(
        "La différence entre le carré de la somme des cent premiers entiers et 
        entre la somme des carrés des cent premiers entiers est {}",
        res
    );
}
