//Finally got it right.

use std::io::BufferedReader;
use std::io::File;

fn open_file(fp:&str) -> String {
	let path = Path::new(fp);
	let mut file = BufferedReader::new(File::open(&path));
	let read_file = file.read_to_string().unwrap();
	//for some reason the below line doesn't work here:
	//
	//let names:Vec<&str> = read_file.split_str(",").collect();
	//
	//it somehow results in lifetime errors for read_file. I can get around
	//this by just passing back the string instead of making the vector
	//in this function, so I need to split the string at the top of
	//main().
	read_file
}

fn name_score(name:&str) -> uint {
	let mut sum = 0u;
	for a in name.chars() {
		//println!("{}, {}", a, a.to_digit(36));
		match a.to_digit(36) {
			Some(x) => sum += x-9,
			None	=> sum += 0
			}
	}
	sum
}

fn main() {
	let name_str:String = open_file("src/PE022.txt");
	//How do I get rid of the ' " 's?
	let mut names:Vec<&str> = name_str.split_str(",").collect();
	names.sort();
	let mut sum = 0u;
	//let names_test = ["COLIN"];
	for it in names.iter().enumerate() {
		sum += (it.0+1)*name_score(*it.1);
	}
	println!("{}", sum);
}