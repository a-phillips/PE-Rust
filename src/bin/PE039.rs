use std::num::Int;

fn num_int_sols(p:i64) -> i64 {
	let mut c;
	let mut count = 0i64;
	for a in range(1i64, p/3) {
		for b in range(a, p) {
			c = p - a - b;
			if c < b { break; }
			else if c.pow(2) == (a.pow(2) + b.pow(2)) {
				count += 1;
				//println!("{}, {}, {}", a, b, c)
			}
		}
	}
	count
}

fn main() {
	let mut max_count = 0i64;
	let mut max_p = 0i64;
	for i in range(5i64, 1001i64) {
		let curr_count = num_int_sols(i);
		if curr_count > max_count {
			max_count = curr_count;
			max_p = i
		}
	}
	println!("{}", max_p);
}