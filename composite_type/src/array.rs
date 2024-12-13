use std::{array, io};

fn main() {
    //   数组
    // 在日常开发中，使用最广的数据结构之一就是数组，在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的array，
    // 第二种是可动态增长的但是有性能损耗的 Vector，在本书中，我们称 array 为数组，Vector 为动态数组。

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // [类型; 长度]。
    let a: [i32; 3] = [3; 3];

    let first = a[0];
    let second = a[1];
    println!("array! {} {}", first, second);

    // 越界访问
    // let mut index = String::new();
    // // 读取控制台输入
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number ");

    // let element = a[index];

    // println!(
    //     "array! The value of element at index {} is: {}",
    //     index, element
    // );

    //  数组元素为非基础类型
    // 此声明方式是将元素不断的拷贝出来，然而复杂类型不支持拷贝
    // let array = [String::from("8"); 8]; // the trait bound `String: Copy` is not satisfied

    let array = [String::from("8"), String::from("8")]; // it's ok
    println!("array! {:?}", array);

    // 数组切片

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &a[2..5];
    assert_eq!(slice, &[3, 4, 5], "1");

    //     切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
    // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
    // 切片类型 [T] 拥有不固定的大小，而切片引用类型 &[T] 则具有固定的大小，
    // 因为 Rust 很多时候都需要固定大小数据类型，因此 &[T] 更有用，&str 字符串切片也同理

    // 最后，让我们以一个综合性使用数组的例子，来结束本章节的学习：

    let one: [u8; 3] = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];

    let blank1: [u8; 3] = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
    // 借用arrays的元素用作循环中
    for a in &arrays {
        println!("arrays {:?}", a);
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            println!("\t {} + 10 = {}", n, n + 10)
        }

        let mut sum: u8 = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i]
        }
        println!("\t({:?} = {})", a, sum);
    }
}
