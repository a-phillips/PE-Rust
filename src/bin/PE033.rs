fn main() {
	let mut nums = [0i64, ..4];
	let mut dens = [0i64, ..4];
	let mut i = 0u;
	for num in range(10i64, 100i64-1) {
		for den in range(num+1, 100i64) {
			let num_chars:Vec<char> = num.to_string().chars().collect();
			let den_chars:Vec<char> = den.to_string().chars().collect();
			//println!("{}, {}, {}", num, den, num_chars[1] == den_chars[0]);
			if (num_chars[0] == den_chars[1]) {
				if ((num%10) as f64)/((den/10) as f64) == (num as f64)/(den as f64) {
					println!("{}, {}", num, den);
					nums[i] = num;
					dens[i] = den;
					i += 1;
				}
			}
			else if (num_chars[1] == den_chars[0]) {
				if ((num/10) as f64)/((den%10) as f64) == (num as f64)/(den as f64) {
					println!("{}, {}", num, den);
					nums[i] = num;
					dens[i] = den;
					i += 1
				}
			}
		}
	}
	let mut num_prod = 1i64;
	let mut den_prod = 1i64;
	for i in range(0u, 4) {
		num_prod *= nums[i];
		den_prod *= dens[i];
	}
	for i in range(2i64, num_prod/2) {
		while (num_prod%i == 0) && (den_prod%i == 0) {
			num_prod /= i;
			den_prod /= i;
		}
	}
	println!("{}", den_prod);
}