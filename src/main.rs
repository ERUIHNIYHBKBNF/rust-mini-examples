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
    let str2: String = str1.to_string() + varchar.to_string().as_str(); // &String::from(varchar)
    println!("{}", str2); // Hello,{}😀

    let tuple = ("chino", 1204, true);
    println!("({}, {}, {})", tuple.0, tuple.1, tuple.2);
    // <struct>.<field>
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }
    // <tuple>.<index>
    struct Grades(char, char, char, char, f32);
    let student: Student = Student {
        name: String::from("Chino"),
        level: 1,
        remote: true,
    };
    let grades: Grades = Grades('A', 'B', 'C', 'D', 0.9);
    print!("name: {}, level: {}, remote: {} ", student.name, student.level, student.remote);
    println!("{}", grades.0);

    /**
     * 通过 #[derive(Debug)] 语法可以在代码执行期间查看某些在标准输出中无法查看的值。
     * 要使用 println! 宏查看调试数据，请使用语法 {:#?} 以可读的方式格式化数据。
     */
    #[derive(Debug)]
    struct ColorProps {
        red: f32,
        green: f32,
        blue: f32,
    }
    #[derive(Debug)]
    struct ColorTuple(f32, f32, f32);
    /** An enum variant can be like a unit without fields or data types / tuple / classic struct
     * 枚举中的每个变体均不是其自己的类型, 任何使用枚举变体的函数都必须接受枚举中的所有变体
     * 通过为枚举中的每个变量定义单独的结构，可以直接访问特定变量的数据。
     * 感觉enum就像存储一堆struct的类型（？）
     */
    #[derive(Debug)]
    enum Color {
        Hex(String),
        RgbTuple(ColorTuple),
        RgbStruct(ColorProps),
    }

    let red_str: String = "#ff0000".to_string();
    println!("red({})", red_str);
    let red: Color = Color::Hex(red_str);

    let green_tuple = ColorTuple(0.0, 1.0, 0.0);
    println!("green({}, {}, {})", green_tuple.0, green_tuple.1, green_tuple.2);
    let green: Color = Color::RgbTuple(green_tuple);

    let blue_struct: ColorProps = ColorProps {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };
    println!("blue({}, {}, {})", blue_struct.red, blue_struct.green, blue_struct.blue);
    let blue: Color = Color::RgbStruct(blue_struct);

    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("enum:\nred: {:#?}\ngreen: {:#?}\nblue: {:#?}", red, green, blue);


    
}
