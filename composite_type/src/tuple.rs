fn main() {
    //     元组
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。

    // 可以通过以下语法创建一个元组：
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    println!("tuple! {:?}", tup);

    let (x, y, z) = tup;

    println!("tuple! {} {} {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("tuple! {} {} {}", five_hundred, six_point_four, one);

    let s1: String = String::from("hello");

    let (str, len) = calculate_len(s1);
    println!("tuple! {} {}", str, len);
}

fn calculate_len(str: String) -> (String, usize) {
    let len: usize = str.len();
    (str, len)
}
