fn recur1(n: i32) {
    if n > 0 {
        recur1(n - 1);
        print!("{} ", n); // 1 2 3
    }
}

fn recur2(n: i32) {
    if n > 0 {
        print!("{} ", n); // 3 2 1
        recur2(n - 1);
    }
}

fn main() {
    let n = 3;
    recur1(n);

    println!("");

    recur2(n);
    println!("");
}
