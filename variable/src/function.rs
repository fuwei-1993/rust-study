use std::fmt::Debug;

fn main() {
    // 函数要点
    // 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
    // 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    // 每个函数参数都需要标注类型

    fn add(i: i32, j: i32) -> i32 {
        i + j
    }
    println!("5 + 3 = {}", add(5, 3));

    another_function(1, 2.01);

    let x: i32 = plus_5(3);
    println!("x: {}", x);

    println!("plus_or_minus: {}", plus_or_minus(3));
    println!("plus_or_minus: {}", plus_or_minus(6));

    // Rust 中的特殊返回类型

    fn report<T: Debug>(item: T) {
        println!("Item: {:?}", item);
    }

    let x = report(1);
    println!("x: {:?}", x);

    fn clear(text: &mut String) {
        *text = String::from("")
    }
    let text: &mut String = &mut String::new();

    text.push('1');
    text.push_str("world");

    println!("text: {}", text);
    let y = clear(text);

    println!("y: {:?}", y);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_5(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}
