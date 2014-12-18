fn main(){
    let mut SumSq: i64 = 0;
    let mut SqSum: i64 = 0;
    for i in range(1, 101){
        SumSq += i*i;
        SqSum += i;
    }
    SqSum = SqSum*SqSum;
    println!("{}",SqSum);
    println!("{}",SumSq);
    println!("{}",SqSum-SumSq);
}
