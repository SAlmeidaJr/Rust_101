fn main(){
    let n:u32 = 10;
    let res1 = square_of_sum(n);
    let res2 = sum_of_squares(n);
    let res3 = res1 - res2;


    println!("{}",res1);
    println!("{}",res2);
    println!("{}",res3);
}
pub fn square_of_sum(n: u32) -> u32{
    let mut count:u32 = 0;

    for x in 1..=n{
        count += x
    };
    let square_res:u32 = count.pow(2);
    square_res
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut count2:u32 = 0;

    for i in 1..=n{
        count2 += i.pow(2)
    };
    count2
}
