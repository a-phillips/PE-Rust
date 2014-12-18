//This feels like cheating - if I start max_prime_div with div=2
//and skip by 1 in the function's "else" statement, then it
//overflows, but it doesn't if you start at 3 and skip by 2.

fn max_prime_div(num: i64, div: i64) -> i64{
    if num == 1 { return div; }
    else if num%div==0 { return max_prime_div(num/div, div); }
    else { return max_prime_div(num, div+2); }
}

fn main(){
    println!("{}", max_prime_div(600851475143, 3));
}
