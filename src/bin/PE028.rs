fn main() {
	//The numbers on the diagonal increase 4 at a time by 2, then 4, then 6, ...
	//The upper limit is the square of the dimensions, inclusive.
	let dim = 1001u;
	let lim = dim*dim;
	let mut curr_num = 3u;
	let mut inc_by = 2u;
	let mut inc_count = 0u;
	let mut sum = 1u;
	while curr_num <= lim {
		//println!("{}", curr_num);
		sum += curr_num;
		inc_count += 1;
		if inc_count == 4 {
			inc_by += 2;
			inc_count = 0;
		}
		curr_num += inc_by;
	}
	println!("{}", sum);
}