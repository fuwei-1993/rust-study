use core::slice;

fn main() {
    let my_name = "fu_wei";
    // greet(my_name); error expect String found &str

    // 切片(slice)
    let s: String = String::from("hello world");

    //  左闭右开区间
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    // rust 中的range 语法表明凡是我们没有显式声明的字段，全部从
    let s: String = String::from("hello");
    // 等效的写法
    let slice: &str = &s[0..2];
    let slice: &str = &s[..2];

    let len: usize = s.len();
    // 等效的写法
    let slice: &str = &s[4..];
    let slice: &str = &s[4..len];
    // 你也可以截取整个字符串
    let slice = &s[..len];
    println!("slice: {}", slice);
    let slice = &s[..];
    println!("slice: {}", slice);

    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，
    // 例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：
    let s: &str = "中国人";
    // let a: &str = &s[0..2];

    // println!("slice: {}", a); // byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中国人

    let mut s = String::from("hello world");
    let word: &str = first_word(&s);

    s.clear();
    // 当我们已经有了可变借用时，就无法再拥有不可变的借用。因为 clear 需要清空改变 String，因此它需要一个可变借用（利用 VSCode 可以看到该方法的声明是 pub fn clear(&mut self) ，参数是对自身的可变借用 ）；
    // 而之后的 println! 又使用了不可变借用，也就是在 s.clear() 处可变借用与不可变借用试图同时生效，因此编译无法通过。
    // println!("The first word is: {}", word); // cannot borrow `s` as mutable because it is also borrowed as immutable

    // 其它切片
    let a = [1, 23, 4, 5];
    let slice = &a[1..2];
    assert_eq!(slice, &[23]);

    // 字符串字面量是切片
    let s: &str = "Hello, world!";

    // 什么是字符串?
    // 顾名思义，字符串是由字符组成的连续集合，但是在上一节中我们提到过，Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    // 但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间

    // String 与 &str 的转换
    // String::from("hello,world")
    // "hello,world".to_string()

    // 那么如何将 String 类型转为 &str 类型呢？答案很简单，取引用即可：
    let s = String::from("hello, world");
    say_hello(&s[..]);
    say_hello(&s);
    say_hello(s.as_str());

    // 深入字符串内部
    // 字符串的底层的数据存储格式实际上是[ u8 ]，一个字节数组。对于 let hello = String::from("Hola"); 这行代码来说，Hola 的长度是 4 个字节，
    // 因为 "Hola" 中的每个字母在 UTF-8 编码中仅占用 1 个字节，但是对于下面的代码呢？

    // let hello = String::from("中国人");
    // 每个汉字占用3个字节， 所以取 &s[0..2] 没有意义

    // 操作字符串
    // 追加 (Push)
    let mut s = String::from("hello ");
    s.push_str("rust");
    println!("{}", s);
    s.push('!');
    println!("{}", s);

    // 插入 (Insert)
    let mut s = String::from("hello rust!");
    s.insert(5, ',');
    println!("{}", s);
    s.insert_str(s.len(), " I like it");
    println!("{}", s);

    // 替换 (Replace)
    let s: String = String::from("I  like rust. Learning rust is my favorite!");
    let new_s: String = s.replace("rust", "Rust");
    println!("{}", new_s);
    // 2、replacen
    let new_s: String = s.replacen("rust", "Rust", 1);
    println!("{}", new_s);

    // 3、replace_range
    // 该方法是直接操作原来的字符串，不会返回新的字符串
    let mut s: String = String::from("你I  like rust. Learning rust is my favorite!");
    s.replace_range(0..3, "你好");
    println!("{}", s);

    // 删除 (Delete)
    // 与字符串删除相关的方法有 4 个，它们分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String 类型

    // 1、 pop —— 删除并返回字符串的最后一个字符
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None。
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
fn say_hello(s: &str) {
    println!("Hello, {}!", s);
}
