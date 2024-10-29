fn main(){
    let res = square(0);
    println!("{}",res);
    println!("{}",total());
}

pub fn square(s: u32) -> u64 {
    let mut count:u64 = 1;
    match s{
        1=>1_u64,
        0=>panic!(),
        _=>{
            for _x in 2..=s{
                count *= 2
            };
           return count
        }
    }
    
}

pub fn total() -> u64 {
    let mut tt:u64 = 1;
    let mut totalis = 0;
    for ii in 1..=64{
        if ii == 1{
            totalis += 1;
        }else{
            tt *= 2;
            totalis += tt;
        }
    }
    totalis
}