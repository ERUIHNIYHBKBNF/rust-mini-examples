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

    
}
