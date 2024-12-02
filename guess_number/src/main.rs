use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("猜数游戏");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("猜测一个数");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("无法执行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小啦"),
            Ordering::Greater => println!("太大啦"),
            Ordering::Equal => {
                println!("猜对啦");
                break;
            }
        }
    }
}
