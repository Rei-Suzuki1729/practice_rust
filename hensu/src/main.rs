const PAGE_SIZE: usize = 4096;
static VERSION: u64 = 745;
fn main() {
    let y = 39;
	let mut x = 69;
    println!("y = {}",y);
	x = x + y;
	println!("x = {}",x);
	println!("PAGE_SIZE = {}",PAGE_SIZE);
	println!("VERSION = {}",VERSION);
}

