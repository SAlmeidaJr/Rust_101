use std::io;
fn main() {
    let mut multiplo = String::new();
    println!("coloque um numero para mostrar seus multiplos: ");
    io::stdin()
    .read_line(&mut multiplo)
    .expect("erros acontecem");

    let multiplo:i32 = multiplo.trim().parse().expect("error");
    
    for x in (multiplo..=(multiplo*10)).step_by(multiplo as usize){
        println!("{}",x);
    };
}
