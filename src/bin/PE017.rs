fn main() {
	let base_nums = [3u, //one
					3, //two
					5, //three
					4, //four
					4, //five
					3, //six
					5, //seven
					5, //eight
					4];//nine
	let teens = [3u, //ten
				6, //eleven
				6, //twelve
				8, //thirteen
				8, //fourteen
				7, //fifteen
				7, //sixteen
				9, //seventeen
				8, //eighteen
				8]; //nineteen
	let tens_prefix = [6u, //twenty
						6, //thirty
						5, //forty
						5, //fifty
						5, //sixty
						7, //seventy
						6, //eighty
						6]; //ninety
	let hundreds_prefix = [13u, //one hundred and
							13, //two hundred and
							15, //three hundred and
							14, //four hundred and
							14, //five hundred and
							13, //six hundred and
							15, //seven hundred and
							15, //eight hundred and
							14];//nine hundred and
	let one_K = 11u; //one thousand
	let mut mut_sum = 0u;
	//get count under ten
	for i in range(0u, base_nums.len()) {
		mut_sum += base_nums[i];
	}
	let count_under_ten = mut_sum;
	mut_sum = 0;
	//get count in teens
	for i in range(0u, teens.len()) {
		mut_sum += teens[i];
	}
	let count_in_teens = mut_sum;
	//get count in 20+
	mut_sum = 0;
	for i in range(0u, tens_prefix.len()) {
		mut_sum += ((base_nums.len()+1)*tens_prefix[i]) + count_under_ten;
	}
	let count_over_20 = mut_sum;
	//calc count per hundred
	let count_per_hundred = count_under_ten + count_in_teens + count_over_20;
	//use calc_per_hundred with the hundreds_prefixes to get count_over_100
	mut_sum = 0;
	for i in range(0u, hundreds_prefix.len()) {
		mut_sum += hundreds_prefix[i] - 3; //___ hundred (no "and")
		mut_sum += (99*hundreds_prefix[i]) + count_per_hundred;
	}
	let count_over_100 = mut_sum;
	//answer will be count over 100 + count per 100 (for under 100) + one thousand
	let answer = count_over_100 + count_per_hundred + one_K;
	println!("{}",answer);
}