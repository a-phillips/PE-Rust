use std::num::Float;

fn num_divs(num:i64) -> int{
	let ilim = ((num as f64).sqrt() as i64)+1;
	let mut count = 2i;
	for i in range(2i64, ilim) {
		if num%i==0 {
			if num/i == i { count += 1; }
			else { count += 2; }
		}
	}
	count
}

fn main(){
	let mut num = 0i64;
	let mut add = 1i64;
	loop {
		num += add;
		if num_divs(num) >= 500 { break; }
		else { add += 1; }
	}
	println!("{}",num);
}