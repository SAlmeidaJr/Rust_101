
fn main() {
    let num:u32 = 1;

    let mut vec : Vec<u32> = Vec::new();

    let quant = num.to_string().len() as u32;

    for x in num.to_string().chars() {

      let calculo: u32 = x.to_digit(10).unwrap();

      vec.push(calculo.pow(quant));

    };
    let soma:u32 = vec.iter().sum();
    println!("{}",num == soma);

}
