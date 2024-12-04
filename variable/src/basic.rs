fn main() {
    //     字符类型(char)
    // 字符，对于没有其它编程经验的新手来说可能不太好理解（没有编程经验敢来学 Rust 的绝对是好汉），但是你可以把它理解为英文中的字母，中文中的汉字。
    let c: char = 'z';
    let g: char = '国';
    println!("c: {}, g: {} \n", c, g);

    //由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
    let z: char = '中';
    print!(
        "字符'中'占用了{}字节的内存大小 \n",
        std::mem::size_of_val(&z)
    );

    // bool 布尔类型
    let f: bool = false;
    if f {
        print!("不会打印的字符串")
    }

    // 单元类型 ？？wtf ?
    let unit: () = ();

    print!("unit: {:?} \n", unit);
}
