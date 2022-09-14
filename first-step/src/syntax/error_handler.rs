pub fn main() {
    // panic!("Farewell!"); // panic!宏使当前线程panic

    // Option类型处理缺失
    /*
    enum Option<T> {
        None,     // The value doesn't exist
        Some(T),  // The value exists
    }
    可以不需要Option::前缀直接使用Some和None
    只要一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空。
    */
    let fruits = vec!["banana", "apple", "coconut"];

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_existent = fruits.get(98);
    println!("{:?}", non_existent);

    // 使用match匹配枚举中的值来操作控制流(箭头返回值即为表达式的值)
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            // 执行第一个匹配上的 match arm
            Some(&"banana") => println!("Found a banana at index {}", index),
            Some(fruit) => println!("Fruit at index {} is {}", index, fruit),
            None => println!("There's no fruit at index {}", index),
            // _ => println!("Something else"),
        }
    }

    // if let 表达式，你关注的是要匹配的单个模式时，你不需要 match 表达式的所有样板代码
    let mut a = Some(2);
    if let Some(1) = a {
        println!("a is 1");
    } else {
        println!("a is not 1");
    }

    // 使用unwrap方法来获取Option中的值，None会panic
    println!("a is {}", a.unwrap());
    a = None;
    println!("None is {}", a.unwrap_or(233));

    // 使用expect，自定义panic信息
    // println!("a is {}", a.expect("a is None"));

    // 使用Result类型来处理可能的错误
    /*
    enum Result<T, E> {
        Ok(T):  // A value T was obtained.
        Err(E): // An error of type E was encountered instead.
    }
    */
    #[derive(Debug)]
    struct DivisionByZeroError;

    fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
        if divisor == 0.0 {
            Err(DivisionByZeroError)
        } else {
            Ok(dividend / divisor)
        }
    }

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{}", safe_division(4.0, 0.0).unwrap_or(0.0));
    println!("{}", safe_division(5.0, 2.0).unwrap());
}
