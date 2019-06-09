pub fn solve() {
    let mut triplets = Vec::new();
    for a in 1..1001 {
        for b in 1..(1001 - a) {
            triplets.push((a, b, 1000 - a - b));
        }
    }
    let mut res = -1;
    for (a, b, c) in triplets.iter() {
        if a * a + b * b == c * c {
            res = a * b * c;
            break;
        }
    }
    println!(
        "Le produit des éléments du triplet pythagoriciene est dont la somme est 1000 est {}",
        res
    );
}
