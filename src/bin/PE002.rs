fn main(){
    let mut a: int = 1;
    let mut b: int = 1;
    let mut c: int = 1;
    let mut total: int = 0;
    let lim: int = 4000000;
    while b < lim{
        if (b%2) == 0{
            total = total + b
        }
        c = a+b;
        a = b;
        b = c;
    }
    println!("{}",total);
}
