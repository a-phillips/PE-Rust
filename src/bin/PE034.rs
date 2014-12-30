fn factorial(n:uint) -> uint {
	range(1u, (n+1)).fold(1u, |prod, i| { prod * i })
}

fn check(num:uint) -> bool {
	let fac_sum = num.to_string()
					.chars()
					.fold(0u, |acc, i| { acc + factorial(i.to_digit(10).unwrap())});
	fac_sum == num
}

fn main() {
	//There is an upper limit at 2903040 = 8*(9!) since the maximum 8-digit
	//factorial sum is less than 8 digits.
	let answer = range(3u, 2903040).filter(|x| check(*x))
									.fold(0u, |acc, i| { acc+i });
	println!("{}", answer);
}