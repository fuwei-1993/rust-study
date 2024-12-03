pub(crate) fn main() {
    // 代表可以修改的变量
    let mut v1: i32 = 1;
    print!("The value of v1 is: {}", v1);
    v1 = 2;
    print!("The value of v1 is: {}", v1);

    // 下划线代表不用的变量
    let _v2: i32 = 2;

    // 变量的结构
    let (a, mut b): (bool, bool) = (true, false);
    print!("The value of a is: {}", a);
    print!("The value of b is: {}", b);
    b = true;
    assert_eq!(b, a);
    print!("The value of b is: {}", b);

    // 变量的遮蔽
    let x: i32 = 5;
    // 作用域 x = 5
    let x: i32 = x + 1;

    println!("The value of x is: {}", x); // 6;

    // 作用域
    {
        let y: i32 = 3;
        println!("The value of y is: {}", y);
    }

    // 字符串类型
    let spaces: &str = "   ";
    let space_len: usize = spaces.len();

    println!("The length of the string is: {}", space_len);

    // 数字 _ 方便看
    let num: i32 = 1_0;
    print!("The value of num is: {}", num);

    // NaN
    let nan: f64 = (-42.0_f64).sqrt();
    print!("The value of nan is: {}", nan);

    // 循环
    for i in 1..5 {
        println!("The current number is: {}", i);
    }

    // 字符
    let x: &str = "😊"; //+
    println!("The length of the string is: {}", x);

    // 函数表达式
    fn add_with_extra(x: i32, y: i32) -> i32 {
        let x: i32 = x + 1;
        let y: i32 = y + 1;
        x + y
        // return x + y // 效果是一样的
    }
    println!("The sum of 1 and 2 is: {}", add_with_extra(1, 2));

    // 表达式, 表达式后面不能加符号
    let y = {
        let x: i32 = 3;
        x * 2
    };
    println!("The value of y is: {}", y);
}
