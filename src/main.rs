fn main() {
    println!("Magic Sums:");
    for n in 3..10 {
        println!("f({}) = {}", n, magic_sum(n));
    }
}

fn magic_sum(n: i32) -> i32 {
    (n.pow(4) + n.pow(2)) / (2*n)
}

/* So the main idea, then, is to do something interesting with this. Perhaps
grids, board games, etc. Who knows, currently? Not me. I'll figure it out
or I won't. We'll see.
*/
