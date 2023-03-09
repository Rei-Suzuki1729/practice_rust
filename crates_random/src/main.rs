use oorandom;
fn main() {
    let some_seed = 4;
    let mut rng = oorandom::Rand32::new(some_seed);
    println!("Your random number is: {}", rng.rand_float());
}
