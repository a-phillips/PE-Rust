fn main() {
    println!("{}", get_answer());
}

fn get_answer() -> i64 {
    for c in range(333i, 1000i){
        for b in range(1i, 1000i-c){
            for a in range(1000i-c-b, 1000i-c-b+1){
                if (a*a)+(b*b) == (c*c) {
                    return (a*b*c) as i64;
                }
            }
        }
    }
    return 0;
}
