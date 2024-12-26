use core::slice;
use std::result;

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
    let mut s: String = String::from("rust pop 中文 1你好");
    let string_pop: Option<char> = s.pop();
    let string_pop: Option<char> = s.pop();
    dbg!(string_pop);

    // 2、 remove —— 删除并返回字符串中指定位置的字符
    let mut remove_string: String = String::from("测试remove string");
    let remove_char: char = remove_string.remove(0);
    println!("{}", remove_string);
    dbg!(remove_char);

    // 3、truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    let mut truncate_string: String = String::from("测试truncate string");
    truncate_string.truncate(3);
    println!("{}", truncate_string);

    // 4、clear —— 清空字符串
    let mut clear_string: String = String::from("测试clear string");
    clear_string.clear();
    println!("clear: {}", clear_string);

    //连接 (Concatenate)
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // string_append 的所有权被转移走了，之后再不能使用它
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("{}", result);

    // format! 这种方式适用于 String 和 &str
    let s1: &str = "hello";
    let s2: String = String::from("rust");
    let s: String = format!("{} {}!", s1, s2);
    println!("{}", s);

    // 字符串转义
    // 我们可以通过转义的方式 \ 输出 ASCII 和 Unicode 字符。
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape: &str = "I am writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_code_point: &str = "\u{211D}";
    let character_name: &str = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_code_point, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string: &str = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes: &str = r#"A "double quoted" string"#;
    println!("{}", quotes);
    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 操作 UTF-8 字符串
    // 字符
    // 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法，例如：
    for c in "中国人".chars() {
        println!("{}", c);
    }
    // 字节
    // 这种方式是返回字符串的底层字节数组表现形式：
    for c in "中国人".bytes() {
        println!("{}", c);
    }

    // 字符串深度剖析
    // 那么问题来了，为啥 String 可变，而字符串字面值 str 却不可以？
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，
    // s 不再有效，内存被释放
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

// fn add(self, s: &str) -> String {
//     self + s
// }
