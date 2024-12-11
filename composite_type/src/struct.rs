#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    // 结构体
    //     上一节中提到需要一个更高级的数据结构来帮助我们更好的抽象问题，结构体 struct 恰恰就是这样的复合数据结构，
    // 它是由其它数据类型组合而来。 其它语言也有类似的数据结构，不过可能有不同的名称，例如 object、 record 等。

    // 结构体跟之前讲过的元组有些相像：都是由多种类型组合而成。但是与元组不同的是，
    // 结构体可以为内部的每个字段起一个富有含义的名称。因此结构体更加灵活更加强大，你无需依赖这些字段的顺序来访问和解析它们。

    let mut user: User = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user.active = false;

    println!(
        "User: {}, {}, {}, {}",
        user.active, user.username, user.email, user.sign_in_count
    );

    // 结构赋值法
    // 因为 user2 仅仅在 email 上与 user1 不同，因此我们只需要对 email 进行赋值，剩下的通过结构体更新语法 ..user1 即可完成。

    // .. 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用。
    let user2: User = User {
        email: String::from("fuwei@example.com"),
        ..user
    };

    println!(
        "User2: {}, {}, {}, {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    let user3: User = User {
        active: false,
        ..build_user(String::from("fuwei2@example.com"), String::from("fuwei"))
    };

    println!(
        "User3: {}, {}, {}, {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );
    // username string 被转移
    // println!("User 3: {}", user.username)
    println!("User 3: {}", user.active);
    // username string 被转移
    // println!("User 1: {:?}", user);

    println!("User 3: {:?}", user3);

    // 结构体的内存排列
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("f1 ${:?}", f1);
    println!("fname {}, f1_length {}", f1_name, f1_length);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
