fn main() {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    println!("days: {:?}", days);
    // Declare array, initialize all values to 0, length = 5
    let bytes: [u32; 5] = [0; 5];
    println!("bytes[4]: {:?}", bytes[4]);

    let vector = vec![1, 2, 3, 4u64, 5];
    println!("vector: {:?}", vector);
    // 可变vector
    let mut fruits = Vec::new();
    fruits.push("Apple");
    fruits.push("Banana");
    fruits.pop();
    fruits.push("Cherry");
    let mut cherry = fruits[1].to_string();
    cherry += "s";
    fruits[1] = &cherry;
    println!("fruits[1]: {}", fruits.pop().unwrap());
}
