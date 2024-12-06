fn main() {
    // 条件语句
    let a: bool = false;
    if a == true {
        print!("world")
    } else {
        print!("hello")
    }

    let condition: bool = true;
    let number = if condition { 1 } else { 2 };
    println!("The number is: {}", number);

    // 循环
    for i in 0..10 {
        println!("Hello, {}", i);
    }
    for i in 0..=10 {
        println!("Hello, {}", i);
    }

    let a = [1, 3, 5, 6, 7];
    for i in a.iter().enumerate() {
        println!("i: {:?}", i); // 单元类型

        println!("Index: {}, Value: {}", i.0, i.1);
    }

    let collection = [1, 3, 4, 5, 6];
    for i in 0..collection.len() {
        println!("Index: {}, Value: {}", i, collection[i]);
    }

    for item in collection {
        println!("Value: {}", item);
    }

    // continue
    for i in 0..5 {
        if i == 2 {
            continue;
        }
        println!("continue, {}", i);
    }
    // break
    for i in 0..5 {
        if i == 2 {
            break;
        }
        println!("break, {}", i);
    }

    // while
    let mut i = 0;
    while i < 5 {
        println!("while, {}", i);
        i += 1;
    }

    // loop
    let mut i: i32 = 0;

    loop {
        if i == 5 {
            println!("loop, {}", i);
            break;
        }
        i += 1
    }

    let mut i: i32 = 0;
    let result: i32 = loop {
        if i == 5 {
            break i + 2;
        };
        i += 1
    };

    println!("result: {}", result);
}
