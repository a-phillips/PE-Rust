fn is_palindrome(num: int) -> bool{
    let num_str = num.to_string();
    let rev_num_str: String = num_str.as_slice().chars().rev().collect();
    return rev_num_str == num_str;
}

fn get_answer() -> int{
    for i in range(100i*100i,999i*999i).rev(){
        if is_palindrome(i) {
            for div in range(100, 999){
                if (i%div==0) && (i/div<1000){
                    return i;
                }
            }
        }
    }
    return 0;
}

fn main(){
    println!("{}", get_answer());
}
