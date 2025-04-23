static mut X: i32 = 0;

fn unsafe_recur(n: i32) -> i32 {
    unsafe {
        if n > 0 {
            X += 1;
            return unsafe_recur(n - 1) + X;
        }
    }
    0
}

fn safe_recur(n: i32, x: &mut i32) -> i32 {
    if n > 0 {
        *x += 1;
        return safe_recur(n - 1, x) + *x;
    }
    0
}

fn main() {
    let mut x = 0;
    println!("{}", safe_recur(5, &mut x)); // 25

    println!("{}", unsafe_recur(5)); // 25
}
