use std::num::Int;

fn n_choose_r (n:int, r:int) -> i64 {
	//To avoid overflow, the result is calculated one numerator and denominator at a time. This
	//way the result may stay low - calculating the entire numerator first will result in huge
	//values. Also, we can eliminate r! or (n-r)! from the numerator, whichever is larger, so
	//we just start our numerator loop at (max(r, n-r) + 1), and our demoninator counter at 1.
	let mut den = 1f64;
	let mut result = 1f64;
	let max_den = if r > (n-r) { r } else { n-r }; //get the bigger of r! or (n-r)!
	for num in range((max_den+1), (n+1)) {
		result *= (num as f64)/den;
		//println!("{}, {}, {}", num, den, result);
		den += 1.0;
	}
	result as i64
}

fn main() {
	//If we tilt the grid into a diagonal, then cut it in half horizontally, then we get a top half
	//that spreads, then filters down to one point. The number of paths to any point is the sum of the
	//number of paths to the 2 points above it - aka it is nCr, where n is how far down and r is how
	//far left to right. We can then solve for all half-way points. The other half is simply a mirror
	//image of the top half, so we know exactly how many paths there are to the bottom point - the same
	//as there were from the top to the middle point. So we just square the value we get at each
	//middle point and sum them.
	let answer = range(0i, (20+1)).map(|x| n_choose_r(20, x).pow(2)).fold(0i64, |acc, i| { acc + i});
	println!("{}", answer);
}