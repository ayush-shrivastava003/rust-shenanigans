// entry point
fn main () {
	// declare constant x as 32bit signed integer with value 10
	let x : i32 = 10;
	// decalre mutable y as 32bit signed integer with value 0
	let mut y : i32 = 0;
	// infinite loop
	loop {
		// add x to y
		y += x;
		// check for y being above 30
		if y > 30 {
			// exits the loop
			break;
		}
	}
}