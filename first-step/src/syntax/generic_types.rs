pub fn main() {
    // 泛型数据类型
    #[derive(Debug)]
    struct P<T, U> {
        x: T,
        y: U,
    }
    let a = P { x: 0, y: true };
    let b = P { x: a.y, y: a.x };
    println!("{:?}", a);
    println!("{:?}", b);

    // 使用特征定义共享行为（类似接口）
    trait Area {
        fn area(&self) -> f64;
    }

    // 为特定类型实现Area特征
    struct Circle {
        radius: f64,
    }
    impl Area for Circle {
        fn area(&self) -> f64 {
            use std::f64::consts::PI;
            PI * self.radius.powf(2.0)
        }
    }
    let circle = Circle { radius: 2.0 };
    println!("Circle area: {}", circle.area());

    struct Rectangle {
        width: f64,
        height: f64,
    }
    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Rectangle area: {}", rectangle.area());

    // 使用derive，由编译器自动实现某些特征
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };
    if p1 == p2 {
        println!("equal!");
    } else {
        println!("not equal!");
    }
    println!("{:?}", p1);
    // 自行实现Display特征以自定义打印
    use std::fmt;
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    println!("{}", p1);

    // 使用特征边界和泛型函数
    // 定义一个将值序列化为Json的特征
    trait AsJson {
        fn as_json(&self) -> String;
    }
    // 函数接受使用该特征的所有类型
    fn send_data_as_json(value: &impl AsJson) {
        println!("{}", value.as_json());
    }
    // 显示声明类型必须实现AsJson特征的函数定义（同上）
    // fn send_data_as_json<T: AsJson>(value: &T) {}
    struct Person {
        name: String,
        age: u8,
        favorite_fruit: String,
    }
    impl AsJson for Person {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
                self.name, self.age, self.favorite_fruit
            )
        }
    }
    let person = Person {
        name: "John".to_string(),
        age: 30,
        favorite_fruit: "apple".to_string(),
    };
    send_data_as_json(&person);

    // 创建和使用迭代器
    /*
        所有迭代器都会实现以下特征
        trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
    */
    #[derive(Debug)]
    struct Counter {
        length: usize,
        count: usize,
    }
    // 先自定义一个new方法
    impl Counter {
        fn new(length: usize) -> Counter {
            Counter { count: 0, length }
        }
    }
    // 实现Iterator特征
    impl Iterator for Counter {
        type Item = usize; // 迭代器的元素类型
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.length {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    let mut counter = Counter::new(6);
    println!("Counter just created: {:#?}", counter);
    for i in 1..7 {
        assert_eq!(counter.next(), Some(i));
    }
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None); // further calls to `next` will return `None`
    assert_eq!(counter.next(), None);
    println!("Counter exhausted: {:#?}", counter);
    // 实现迭代器特征后可以使用for循环访问
    for i in Counter::new(6) {
        println!("{}", i);
    }
    // 一些在next基础上构建的方法可以使用
    let sum: usize = Counter::new(6).sum();
    println!("Sum of 1 to 6: {}", sum);
}
