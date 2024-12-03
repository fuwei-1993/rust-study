// mod variable;

fn main() {
    // variable::main();
    // 整型溢出
    print!("num \n");
    let a: u8 = 255;
    let b: u8 = a.wrapping_add(1);

    println!("a: {}, b: {}", a, b); // 255, 0

    print!("max: {} \n", u8::MAX);
    print!("max: {} \n", u8::max_value());

    // 浮点类型数字 是带有小数点的数字，在 Rust
    // 中浮点类型数字也有两种基本类型： f32 和 f64，分别为 32 位和 64 位大小。
    // 默认浮点类型是 f64，在现代的 CPU 中它的速度与 f32 几乎相同，但精度更高。

    let x: f64 = 2.01;
    let y: f32 = 1.0;

    print!("x: {}, y: {}", x, y);

    // 浮点数根据 IEEE-754 标准实现。f32 类型是单精度浮点型，f64 为双精度。

    // 断言0.1 + 0.2与0.3相等
    // assert_eq!(0.1 + 0.2, 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // NAN

    let x: f32 = (-232.3_f32).sqrt();
    if x.is_nan() {
        println!("x is nan");
    }
    // assert_eq!(x, x)

    // 位运算
    // 说明:
    // & 位与	相同位置均为1时则为1，否则为0
    // | 位或	相同位置只要有1时则为1，否则为0
    // ^ 异或	相同位置不相同则为1，相同则为0
    // ! 位非	把位中的0和1相互取反，即0置为1，1置为0
    // << 左移	所有位向左移动指定位数，右位补0
    // >> 右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）

    // 二进制为 0000010
    let a: u8 = 2; // 也可以写作 let a: u8 = 0b_000_0010

    // 二进制为 0000011
    let b: u8 = 3;

    print!("a value is {:08b} \n", a);
    print!("b value is {:08b} \n", b);
    print!("a & b value is {:08b} \n", a & b);
    print!("a | b value is {:08b} \n", a | b);
    print!("a ^ b value is {:08b} \n", a ^ b);
    print!("!b value is {:08b} \n", !b);
    print!("b << a value is {:08b} \n", b << a);
    print!("b >> 1 value is {:08b} \n", b >> 1);

    let mut a: u8 = a;

    // a 左移 b 位的结果赋值给 a
    a <<= b;
    print!("(a << b) value is {:08b}", a);

    // 序列(Range)

    for i in 0..10 {
        println!("i = {}", i);
    }
    for i in 'a'..='z' {
        println!("i = {}", i);
    }
}
