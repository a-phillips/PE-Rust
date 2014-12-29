//I don't like this brute-force solution - a sieve of sorts is for sure faster
use std::num::Float;

fn is_prime(num:i64) -> bool {
	let lim = ((num as f64).sqrt() as i64) + 1;
	for i in range(2i64, lim) {
		if num % i == 0 { return false; }
	}
	return true;
}

fn main() {
	let mut check_num = 1i64; // since 2 is prime and adding by 2 when stepping into loop
	let mut count = 1i; //since 2 is prime
	while count != 10001i {
		check_num += 2;
		//println!("{}, {}", count, check_num)
		if is_prime(check_num) { count += 1; }
	}
	println!("{}", check_num);
}