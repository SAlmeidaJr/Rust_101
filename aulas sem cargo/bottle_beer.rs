fn main(){
    let a = verse(3);
    let b = sing(2,0);
    println!("{:?}",b);
    println!("_______");
    println!("{}",a);
}
pub fn verse(n: u32) -> String{
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.",n,n,n-1).to_string()
}
pub fn sing(start: u32, end: u32) -> String {
    let mut res = Vec::new();

    for x in (end..=start).rev(){
        match x{
            2 => {
                let line = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", x, x);
                res.push(line);
            },
            1 => {
                let line = format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
                res.push(line);
            },
            _ => {
                let line = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", x, x, x - 1);
                res.push(line);
            }
        }
    }
    res.push("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string());
    let res = res.join("\n");
    res
}
