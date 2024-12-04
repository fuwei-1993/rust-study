fn main() {
    // 语句和表达式
    // Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值，例如
    fn add_with_extra(x: i32, y: i32) -> i32 {
        let x: i32 = x + 1; // 语句
        let y: i32 = y + 1; // 语句
        x + y // 表达式
    }

    print!(" add_with_extra {} \n", add_with_extra(1, 2));

    let a = 8;
    println!("a: {} \n", a);

    // 可连续增长的数组类型
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    println!("a: {}, b: {:?}, c: {} \n", a, b, c);

    // 表达式
    let y: u32 = {
        let x: u32 = 3;
        x + 1
    };
    println!("y: {} \n", y);

    fn ret_unit_type() -> bool {
        // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        // 类似三元运算符，在Rust里我们可以这样写
        let x = 1;
        let y = if x % 2 == 1 { true } else { false };
        y
    }

    println!("ret_unit_type: {} \n", ret_unit_type()); // true
}
