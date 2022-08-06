fn divide_by_5(num: u32) -> u32 {
    if num < 5 {
        return 0;
    }
    // 使函数中的最后一行代码等于要返回的值，从而在函数末尾返回一个值
    num / 5
}

pub fn main() {
    println!("{}", divide_by_5(233));

    // if 可以当作表达式，每个分支必须返回相同类型
    let num = 500;
    let out_of_range = if num < 0 {
        true
    } else if num == 0 {
        true
    } else if num > 512 {
        true
    } else {
        false
    };
    println!("{} is between 0 and 512: {}", num, out_of_range);

    loop {
        println!("loop forever unless break.");
        break;
    }

    // loop可作为表达式，使用break来返回值
    /*
        1. 声明 stop_loop 变量。
        2. 指示程序将变量值绑定到 loop 表达式的结果。
        3. 启动循环。 执行 loop 表达式主体中的操作至break语句。
        4. 将 stop_loop 值设置为 counter 值，这是 loop 表达式的结果。
    */
    let mut counter = 1;
    let stop_loop: i32 = loop {
        counter *= 2;
        if counter > 100 {
            break counter;
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);

    counter = 1;
    while counter <= 3 {
        println!("we loop a while, counter = {}", counter);
        counter += 1;
    }

    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("{}", fruit);
    }

    for number in 0..5 {
        println!("{}", number);
    }
}
