fn recur(n: i32) {
    if n > 0 {
        recur(n - 1);
        print!("{} ", n);
    }
}

fn main() {
    let n = 3;
    recur(n);
}
