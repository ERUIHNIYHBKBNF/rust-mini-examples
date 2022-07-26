fn divide_by_5(num: u32) -> u32 {
	// 使函数中的最后一行代码等于要返回的值，从而在函数末尾返回一个值
    num / 5
}

fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}
