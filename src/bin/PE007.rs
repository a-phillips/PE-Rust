//I don't like this brute-force solution - a sieve of sorts is for sure faster
use std::num::Float;

//EDIT: below is the original is_prime function:
//fn is_prime(num:i64) -> bool {
//	let lim = ((num as f64).sqrt() as i64) + 1;
//	for i in range(2i64, lim) {
//		if num % i == 0 { return false; }
//	}
//	return true;
//}
//
//I'm replacing that with this faster function which uses the fact that all primes > 3 can be written
//as 6k +/- 1 (although some, like 6(4)+1=25, are not prime, but we don't know when that will be)
//This runs *significantly* faster than the original.

fn is_prime (num:i64) -> bool {
	//The below statements take care of numbers divisible by 2 or 3, which is most
	if num < 2 { return false; }
	else if num < 4 { return true; }
	else if (num%2==0) || (num%3==0) { return false; }
	else {
		let mut factor = 5i64; //6(1)-1
		let lim = ((num as f64).sqrt() as i64)+1;
		while factor < lim {
			if (num%factor==0) || (num%(factor+2)==0) { return false; } //divisible by 6k-1 or 6k+1
			factor += 6;
		}
		return true;
	}
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