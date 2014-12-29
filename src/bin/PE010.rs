//Tried making a sieve, but it overflows the stack. I'm sure there's something to be learned here
//with using the heap, but I have no idea what that entails. Instead I'm using the much slower, brute
//force method.

use std::num::Float;

fn is_prime(num:i64) -> bool {
	//use the fact that all primes over 3 can be written as 6k +/- 1 to make search faster
	//use the fact that we only need to check up to sqrt(num) to search for factor
	if (num<2) || (num == 4) { return false; }
	else if (num==2) || (num==3) { return true; }
	else if (num%2==0) || (num%3==0) { return false; }
	else {
		let mut factor = 5i64;
		let lim = ((num as f64).sqrt() as i64) + 1;
		while factor < lim {
			if (num%factor==0) || (num%(factor+2)==0) { return false ; }
			factor += 6;
		}
	}
	return true;
}

fn main() {
	let lim = 2_000_000i64;
	let answer = range(2i64, lim).filter(|x| is_prime(*x))
								 .fold(0i64, |acc, x| { acc + x });
	println!("{}", answer);
}