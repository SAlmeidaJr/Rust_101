pub fn main() {
    let input: &str = "casa";
    let mut new_word : Vec<char> = Vec::new();
    for x in input.chars().rev() {
        new_word.push(x)
    };
    let res : String = new_word.iter().collect();
    println!("{}",res);
}