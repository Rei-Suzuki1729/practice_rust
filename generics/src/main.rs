fn is_equal<T: Eq>(a: T, b: T) -> bool {
    if a == b {
        true
    } else {
        false
    }
}

fn main() {
    println!("{}", is_equal(1, 1)); // true
    println!("{}", is_equal("1", "1")); // true
}
