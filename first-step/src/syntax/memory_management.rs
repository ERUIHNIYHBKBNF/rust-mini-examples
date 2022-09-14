pub fn main() {
    // 转移所有权是默认行为
    let s = String::from("ChinoChann!");
    println!("{}", s);
    let x = s;
    println!("{}", x);
    // println!("{}", s); // compile error

    // 作为参数传递给函数转移所有权
    fn process_string(s: String) {
        println!("{}", s);
    }
    process_string(x);
    // process_string(x); // compile error

    // 具有Copy特征的简单类型
    fn process_number(x: i32) {
        println!("{}", x);
    }
    let a = 233;
    process_number(a);
    process_number(a); // compile success

    // 手动复制不实现Copy的类型，时间成本昂贵
    let s = String::from("ChinoChann!");
    process_string(s.clone());
    process_string(s); // compile success

    // 通过引用来借用一些值
    fn process_string2(s: &String) {
        println!("{}", s);
    }
    let s = String::from("ChinoChann!");
    let s_ref = &s;
    process_string2(s_ref);
    process_string2(s_ref); // compile success
                            // s_ref.push_str("!"); // compile error
    process_string(s);
    // process_string2(s_ref); // compile error
    // 将原始值声明为可变，使用可变借用&mut，可以修改
    let mut s = String::from("ChinoChann!");
    let s_ref = &mut s;
    // 具有多个不可变引用，具有一个可变引用，只能二选一
    // let s_ref2 = &s; // compile error
    s_ref.push_str("!");
    println!("{}", s);

    // x作为y的引用，使用超出了y的生存期，报错
    // let x;
    // {
    //     let y = 233;
    //     x = &y;
    // }
    // println!("{}", x); // compile error

    // 编译器无法判断返回的引用指x还是y，因此无法确定返回值的生存期
    // fn longest_word(x: &String, y: &String) -> &String {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // fn main() {
    //     let magic1 = String::from("abracadabra!");
    //     let magic2 = String::from("shazam!");

    //     let result = longest_word(&magic1, &magic2);
    //     println!("The longest magic word is {}", result);
    // }

    // 在函数中批注生存期
    // 同一生存期名称'a下，所有参数和返回值预期的生存期相同(传入的引用生存期中较小者)
    fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");
    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    // 在类型中批注生存期
    #[derive(Debug)]
    // 该struct实例的生存期不能超过其字段中的引用的生存期
    struct Highlight<'document>(&'document str);
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    // process_string(text); // compile error in next line
    println!("{:?}", dog);
}
