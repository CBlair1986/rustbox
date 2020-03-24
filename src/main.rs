fn main() {
    println!("Magic Sums:");
    for n in 3..10 {
        println!("f({}) = {}", n, magic_sum(n));
    }
    for n in 3..6 {
        println!("Random square (Order {}): {:?}", n, random_square(n))
    }
}

/// Calculate the sum that each row/column/diagonal of a magic square of order-n would equal.
fn magic_sum(n: i32) -> i32 {
    (n.pow(4) + n.pow(2)) / (2 * n)
}

/// Generate a random (potentially magic) order-n square, with numbers ranging from 1 to n^2.
fn random_square(n: i32) -> Vec<Vec<i32>> {
    let mut rows = Vec::new();

    for _i in 0..n {
        let mut col = Vec::new();
        for _j in 0..n {
            col.push(_i + _j * n + 1);
        }
        rows.push(col);
    }
    return rows;
}

/// Generate sequences of numbers that add up to a certain total, given a starting collection of numbers.
fn magic_sequences(ns: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut delta = target;
    Vec::new()
}