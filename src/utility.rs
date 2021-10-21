pub fn pow(n: i32, power: i32) -> i32 {
	let mut f = 1;
	let mut c = 0;
	loop {
		if c >= power {
			break;
		}
		f *= n;
		c += 1;
	}
	return f;
}