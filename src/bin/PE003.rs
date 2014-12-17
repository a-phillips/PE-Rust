fn main(){
    let mut x: i64 = 600851475143;
    let mut div: i64 = 1;
    while x != 1 {
        div = div + 1;
        while (x%div) == 0 {
            x = x / div;
        }
    }
    println!("{}", div);
}
