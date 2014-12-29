//naive, but it works.

fn d(num:uint) -> uint {
	range(1u, num).filter(|x| num%*x==0).fold(0u, |acc, i| {acc + i})
}

fn main() {
	let mut d_arr = [0u, ..10_000];
	let mut sum = 0u;
	for i in range(2u, 10_000) {
		d_arr[i] = d(i);
	}
	for i in range(2u, 10_000) {
		if (d_arr[i] < 10000) && (i == d_arr[d_arr[i]]) && (i != d_arr[i]) {
			//println!("{}", i);
			sum += i;
		}
	}
	println!("{}", sum);
}