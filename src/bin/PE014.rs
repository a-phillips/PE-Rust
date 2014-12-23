fn collatz_count(num:uint) -> uint{
    if num == 1 { return 1; }
    else if num%2 == 0 { return 1 + collatz_count(num/2); }
    else { return 1 + collatz_count((num*3)+1); }
}

fn main(){
    //This non-mutable solution sucks. I kept getting compile errors
    //stating "counts" (now "counts1") was non-copyable, since it was
    //used to create max_count, so I couldn't use it to iterate over
    //again. The commented code below is my uncompilable main():
    //
    //let lim = 1_000_000u;
    //let counts = range(1u, lim).map(|x| collatz_count(x));
    //let max_count:uint = counts.max().unwrap();
    //for it in counts.zip(range(1u, lim)){
    //    if it.0 == max_count {
    //        println!("{}",it.1);
    //        break;
    //    }
    //}
    //
    //This forced me to create counts twice, since I couldn't figure
    //out how to copy it - once to get the max, then once to get an
    //iterator to figure out the index of the max value:
    let lim = 1_000_000u;
    let counts1 = range(1u, lim).map(|x| collatz_count(x));
    let max_count:uint = counts1.max().unwrap();
    let counts2 = range(1u, lim).map(|x| collatz_count(x));
    for it in counts2.zip(range(1u, lim)){
        if it.0 == max_count {
            println!("{}",it.1);
            break;
        }
    }
}
