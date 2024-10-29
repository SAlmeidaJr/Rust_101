fn main(){
    let mut num: i32 = 1000;
    let mut divisor:i32 = 2;
    let mut vetor:Vec<i32> = Vec::new();
    
    while num > 1{
        match num % divisor{
            0 => {
                vetor.push(divisor);
                num /= divisor;
            }
            _ => {
                divisor += 1;
                
            } 
        }
    }
    println!("{:?}",vetor);
}