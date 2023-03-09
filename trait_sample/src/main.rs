trait Calculatable {
    fn result(self) -> i32;
}

impl Calculatable for i32 {
    fn result(self) -> i32 {
        self // 数値はそのまま返す
    }
}

impl Calculatable for &str {
    fn result(self) -> i32 {
        self.parse().unwrap() // 文字列は変換して返す
    }
}

fn is_equal<T: Calculatable, U: Calculatable>(a: T, b: U) -> bool {
    if a.result() == b.result() {
        true
    } else {
        false
    }
}

fn main() {
    println!("{}", is_equal(1, 1)); // true
    println!("{}", is_equal("1", "1")); // true
    println!("{}", is_equal(1, "1")); // true
}
