fn get_even_fibs(lim:i64, a: i64, b: i64) -> i64{
    let c: i64 = a + b;
    if c > lim { return 0; }
    else if c%2==0 { return c + get_even_fibs(lim, b, c); }
    else { return get_even_fibs(lim, b, c); }
}

fn main(){
    println!("{}",get_even_fibs(4000000, 1, 1));
}
