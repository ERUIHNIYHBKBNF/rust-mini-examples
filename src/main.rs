fn main() {
    let mut a = "Chino";
    println!("Hello {}!", a);
    a = "Kafuu Chino";
    println!("Hello {}!", a);

    let a: u32 = 233;
    let b = 233u64;
    println!("a = {},  b = {}", a, b);
    
    let is_bigger: bool = u64::from(a) > b;
    println!("a is bigger than b: {}", is_bigger);

    let varchar: char = '😀';
    let str1: &str = "Hello,{}";
    // TODO: println!(str1, varchar);
    println!("{}{{{}}}", str1, varchar); // Hello,{}{😀}
    
    
}
