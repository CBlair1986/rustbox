fn main() {
    println!("Magic Sums:");
    for n in 3..10 {
        println!("f({}) = {}", n, magic_sum(n));
    }
    for n in 3..6 {
        println!("Random square (Order {}): {:?}", n, random_square(n))
    }
}

fn magic_sum(n: i32) -> i32 {
    (n.pow(4) + n.pow(2)) / (2 * n)
}

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
/* So the main idea, then, is to do something interesting with this. Perhaps
grids, board games, etc. Who knows, currently? Not me. I'll figure it out
or I won't. We'll see.
*/
