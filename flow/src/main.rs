fn main() {
	if_sample();
	loop_sample();
	while_sample();
	for_sample();
	println!("Done ..");
}
fn if_sample(){

    let n = -5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}
fn loop_sample(){
    let mut count = 0;
	loop{
		count += 1;
		if count % 2 == 1{
			println!("odd");
			continue;
		}
		println!("even");
		if count == 4 {
			break;
		}
	}
	
}

fn while_sample(){
	let mut counter = 0;
	while counter < 30 {
		counter += 1;
	}
	println!("counter = {}",counter);
}

fn for_sample(){
    for i in 1..100 {
		if i % 10 == 0 {
			println!("i = {}",i);
		}
	}
}
