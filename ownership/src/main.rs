fn main() {
    // 所有权
    // 所有的程序都必须和计算机内存打交道，如何从内存中申请空间来存放程序的运行内容，如何在不需要的时候释放这些空间，成了重中之重，也是所有编程语言设计的难点之一。在计算机语言不断演变过程中，出现了三种流派：

    // 垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存，典型代表：Java、Go
    // 手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存，典型代表：C++
    // 通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查

    // 栈(Stack)与堆(Heap)

    // 栈
    // 栈按照顺序存储值并以相反顺序取出值，这也被称作后进先出。想象一下一叠盘子：当增加更多盘子时，把它们放在盘子堆的顶部，当需要盘子时，再从顶部拿走。不能从中间也不能从底部增加或拿走盘子！
    // 增加数据叫做进栈，移出数据则叫做出栈。
    // 因为上述的实现方式，栈中的所有数据都必须占用已知且固定大小的内存空间，假设数据大小是未知的，那么在取出数据时，你将无法取到你想要的数据。

    // 堆
    // 与栈不同，对于大小未知或者可能变化的数据，我们需要将它存储在堆上。
    // 当向堆上放入数据时，需要请求一定大小的内存空间。操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的指针，该过程被称为在堆上分配内存，有时简称为 “分配”(allocating)。
    // 接着，该指针会被推入栈中，因为指针的大小是已知且固定的，在后续使用过程中，你将通过栈中的指针，来获取数据在堆上的实际内存位置，进而访问该数据。
    // 由上可知，堆是一种缺乏组织的数据结构。想象一下去餐馆就座吃饭：进入餐馆，告知服务员有几个人，然后服务员找到一个够大的空桌子（堆上分配的内存空间）并领你们过去。如果有人来迟了，他们也可以通过桌号（栈上的指针）来找到你们坐在哪。

    // 性能区别
    // 在栈上分配内存比在堆上分配内存要快，因为入栈时操作系统无需进行函数调用（或更慢的系统调用）来分配新的空间，只需要将新数据放入栈顶即可。相比之下，在堆上分配内存则需要更多的工作，
    // 这是因为操作系统必须首先找到一块足够存放数据的内存空间，接着做一些记录为下一次分配做准备，如果当前进程分配的内存页不足时，还需要进行系统调用来申请更多内存。 因此，处理器在栈上分配数据会比在堆上分配数据更加高效。

    // 所有权与堆栈
    // 当你的代码调用一个函数时，传递给函数的参数（包括可能指向堆上数据的指针和函数的局部变量）依次被压入栈中，当函数调用结束时，这些值将被从栈中按照相反的顺序依次移除。
    // 因为堆上的数据缺乏组织，因此跟踪这些数据何时分配和释放是非常重要的，否则堆上的数据将产生内存泄漏 —— 这些数据将永远无法被回收。这就是 Rust 所有权系统为我们提供的强大保障。
    // 对于其他很多编程语言，你确实无需理解堆栈的原理，但是在 Rust 中，明白堆栈的原理，对于我们理解所有权的工作原理会有很大的帮助。

    // 所有权原则
    // 理解了堆栈，接下来看一下关于所有权的规则，首先请谨记以下规则：
    // - Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    // - 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    // - 当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)

    {
        // s 在这里无效，它尚未声明
        let s: &str = "hello"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s不再有效

    // 变量绑定背后的数据交互
    // 转移所有权
    let x: i32 = 5;
    let y: i32 = x;
    // 这段代码并没有发生所有权的转移，原因很简单： 代码首先将 5 绑定到变量 x，接着拷贝 x 的值赋给 y，最终 x 和 y 都等于 5，因为整数是 Rust 基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，都被存在栈中，完全无需在堆上分配内存。
    // 整个过程中的赋值都是通过值拷贝的方式完成（发生在栈中），因此并不需要所有权转移。

    let s1 = String::from("hello world ");
    let s2 = s1;
    // println!("{}", s1); //  value borrowed here after move

    // 克隆(深拷贝)
    // 首先，Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。
    // 如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。

    let s1: String = String::from("Hello");
    let s2: String = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 拷贝(浅拷贝)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。
    // - 所有整数类型，比如 u32
    // - 布尔类型，bool，它的值是 true 和 false
    // - 所有浮点数类型，比如 f64
    // - 字符类型，char
    // - 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
    // - 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意：可变引用 &mut T 是不可以 Copy的

    // 函数传值与返回
    let s: String = String::from("hello"); // s进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
    // println!("{}", s); // value borrowed here after move
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，但 i32 是 Copy 的，因此什么也不会发生
    println!("x {}", x); // x 仍然可以在这里使用

    // 返回值与作用域
    let s1 = gives_some_ownership(); // gives_ownership 将返回值移动到 s1

    let s2 = String::from("hello world");
    let s3 = takes_and_gives_back(s2); // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3
    // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    // 所以什么也不会发生。s1 移出作用域并被丢弃

    // 所有权很强大，避免了内存的不安全性，但是也带来了一个新麻烦： 总是把一个值传来传去来使用它。 传入一个函数，很可能还要从该函数传出去，结果就是语言表达变得非常啰嗦，幸运的是，Rust 提供了新功能解决这个问题
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_some_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
