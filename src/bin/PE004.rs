fn is_palindrome(num: int) -> bool{
    let num_str = num.to_string();
    let rev_num_str: String = num_str.as_slice().chars().rev().collect();
    return rev_num_str == num_str;
}

fn main(){
    let mut pals = Vec::new();
    let mut i: int = 999i * 999i;
    while i > (100i*100i) {
        if is_palindrome(i) {
            pals.push(i);
        };
        i = i - 1;
    }
    println!("pals found");
    let mut found: bool = false;
    for pal in pals.iter() {
        for div in range(100, 999) {
            if (*pal%div) == 0 && (*pal/div) < 1000 {
                found = true;
                break;
            };
        }
        if found {
            println!("{}", pal);
            break;
        };
    }
}
