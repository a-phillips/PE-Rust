fn collatz_count(num:uint) -> uint{
    if num == 1 { return 1; }
    else if num%2 == 0 { return 1 + collatz_count(num/2); }
    else { return 1 + collatz_count((num*3)+1); }
}

fn main(){
    let lim = 1_000_000u;
	let mut max_count = 0u;
	let mut max_num = 0u;
	let mut curr_count = 0u;
	for i in range(1u, lim) {
		curr_count = collatz_count(i);
		if curr_count > max_count {
			max_count = curr_count;
			max_num = i;
		}
	}
	println!("{}, {}", max_count, max_num);
}
