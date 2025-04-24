fn recur(n: i32) {
    if n > 0 {
        print!("{} ", n);
        recur(n - 1);
    }
}

fn main() {
    let n = 3;
    recur(n);
}
