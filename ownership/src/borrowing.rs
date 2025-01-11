fn main() {
    // 引用与解引用
    let y = 5;
    let x = &y;
    println!("y = {}", y); // x = 5
    println!("*x = {}", *x); // y = 5
    println!("x = {}", x); // y = 5

    // 不允许比较整数与引用，因为它们是不同的类型。必须使用解引用运算符解出引用所指向的值。
    // assert_eq!(5, x); //  can't compare `{integer}` with `&{integer}`

    // 不可变引用
    let s1: String = String::from("Hello world");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // 如果我们想修改它呢
    let s1 = String::from("Hello,");
    change_immutable_str(&s1);

    // 可变引用
    let mut s1: String = String::from("Hello");
    change_mutable_str(&mut s1);
    println!("mutable string: {}", s1); // Hello, world

    // 可变引用同时只能存在一个
    let mut s1: String = String::from("Hello");
    let r1: &mut String = &mut s1;
    // let r2: &mut String = &mut s1; // cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2);

    // 这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：
    // 1. 两个或更多的指针同时访问同一数据
    // 2. 至少有一个指针被用来写入数据
    // 3. 没有同步数据访问的机制

    // 数据竞争会导致未定义行为，这种行为很可能超出我们的预期，难以在运行时追踪，并且难以诊断和修复。而 Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
    // 很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域：
    let mut s: String = String::from("hello");
    {
        let r1: &mut String = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2: &mut String = &mut s;

    // 可变引用与不可变引用不能同时存在

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // 无法借用可变 `s` 因为它已经被借用了不可变
    println!("{}, {}", r1, r2); // Hello, Hello

    // NLL
    // 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。
    // 虽然这种借用错误有的时候会让我们很郁闷，但是你只要想想这是 Rust 提前帮你发现了潜在的 BUG，其实就开心了，虽然减慢了开发速度，但是从长期来看，大幅减少了后续开发和运维成本。

    // 悬垂引用(Dangling References)
    // 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用。
    // 在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：
    // 当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。
    // 让我们尝试创建一个悬垂引用，Rust 会抛出一个编译时错误：
    let reference_to_nothing = dangle_pointer();
    let reference_to_nothing = dangle_pointer_resolved();

    // 总的来说，借用规则如下：

    // 1. 同一时刻，你只能拥有要么一个可变引用，要么任意多个不可变引用
    // 2. 引用必须总是有效的
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_immutable_str(s: &String) {
    // s.push_str(", world"); // cannot borrow `*s` as mutable, as it is behind a `&` reference
}
fn change_mutable_str(s: &mut String) {
    s.push_str(", world");
}

fn dangle_pointer() {
    // let s = String::from("hello");
    // &s // 该函数返回了一个借用的值，但是已经找不到它所借用值的来源
} //  这里 s 离开作用域并被丢弃。其内存被释放。

// 解决办法是

fn dangle_pointer_resolved() -> String {
    let s = String::from("hello");
    s
}
