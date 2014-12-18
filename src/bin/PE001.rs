fn main(){
    println!("{}",get_answer(1000));
}

fn get_answer(num: int) -> int{
    if num == 1 {
        return 0i;
    }
    else if (num%3==0) || (num%5==0){
        return num + get_answer(num-1);
    }
    else{
        return get_answer(num-1);
    }
}


