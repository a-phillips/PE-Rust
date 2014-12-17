fn main(){
    let mut total = 0i;
    for x in range(1i, 1000i){
        if (x%3 == 0) || (x%5 == 0){
            total = total + x
        }
    }
    println!("{}",total);
}
