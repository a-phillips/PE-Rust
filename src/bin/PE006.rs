use std::num::Int;

fn get_sum_sq(lim: int, i: int) -> int{
    if i > lim { return 0; }
    else { return (i*i) + get_sum_sq(lim, i+1); }
}

fn get_sq_sum(lim: int, i: int) -> int{
    if i > lim { return 0; }
    else if i != 1 { return i + get_sq_sum(lim, i+1); }
    else { return (i + get_sq_sum(lim, i+1)).pow(2); }
}

fn main(){
    println!("{}",get_sq_sum(100, 1) - get_sum_sq(100, 1));
}
