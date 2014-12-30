use std::io::BufferedReader;
use std::io::File;
use std::num::Int;

fn open_file(fp:&str) -> Vec<String> {
	let path = Path::new(fp);
	let mut file = BufferedReader::new(File::open(&path));
	let nums: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	nums
}

fn main() {
	println!("{}",2u64.pow(64)-1);
	let num_digits = 10u;
	let raw_nums = open_file("src/PE013.txt");
	let mut sum = 0u64;
	for str_num in raw_nums.iter() {
		let num:u64 = from_str(str_num.slice(0,num_digits+1)).unwrap();
		sum += num;
	}
	println!("{}", sum.to_string().slice(0,num_digits));
}