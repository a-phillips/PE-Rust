use std::num::Float;

fn is_prime(n:u64) -> bool {
	if n == 3 { return true; }
	if (n%2==0) || (n%3==0) { return false; }
	let lim = ((n as f64).sqrt() as u64) + 1;
	let mut check = 5u64;
	while check < lim {
		if (n%check==0) || (n%(check+2)==0) { return false; }
		check += 6;
	}
	return true;
}

fn main() {
	let mut curr = 1u64;
	let mut inc_by =  0u64;
	let mut ratio = 1f64;
	let mut numerator = 0u64;
	let mut denominator = 1u64;
	while ratio >= 0.1f64 {
		inc_by += 2; // new diagonal, increase increment by 2
		for i in range (0u, 4) { // 4 numbers per diagonal
			curr += inc_by;
			if is_prime(curr) { numerator += 1; }
		}
		denominator += 4; //4 new numbers
		ratio = (numerator as f64)/(denominator as f64);
		//println!("{}, {}", inc_by+1, ratio);
	}
	println!("{}, {}", numerator, denominator);
	println!("{}, {}", inc_by+1, ratio);
}