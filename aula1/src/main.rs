use std::io;
use std::collections::HashMap;
fn main() {

    let mut mapa = HashMap::new();
    loop{

        let mut numeros = String::new();
        
        println!("coloque um numero: ");
        io::stdin()
        .read_line(&mut numeros)
        .expect("falha ao ler");     

        let numeros:i32 = match numeros.trim().parse(){
            Ok(n) => n,
            Err(_) => {
                println!("error happened");
                continue;
            }   
        };

        if numeros == -1 { 
            break;
        };
        if let Some(x) = mapa.get_mut(&numeros){
            *x += 1;
        }else{
            mapa.insert(numeros, 1);
        };
    }
    println!("{:?}",mapa)
}