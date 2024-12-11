// 枚举
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}
fn main() {
    let heart: PokerSuit = PokerSuit::Hearts;
    let diamond: PokerSuit = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    let c1: PokerCard = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2: PokerCard = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };

    println!("Card 1: {:?}", c1);
    println!("Card 2: {:?}", c2);

    // 实现2

    #[derive(Debug)]
    enum PokerSuit2 {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }

    let c1: PokerSuit2 = PokerSuit2::Clubs(1);
    let c2: PokerSuit2 = PokerSuit2::Spades(12);

    println!("Card 1: {:?}", c1);
    println!("Card 2: {:?}", c2);

    let c3: PokerSuit2 = PokerSuit2::Diamonds('A');
    println!("Card 3: {:?}", c3);
    let c4: PokerSuit2 = PokerSuit2::Hearts('B');
    println!("Card 3: {:?}", c4);
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let m1: Message = Message::Move { x: 1, y: 1 };
    let m2: Message = Message::Write(String::from("hello"));
    let m3: Message = Message::ChangeColor(255, 255, 0);
    let m4: Message = Message::Quit;
    println!("m1: {:?}, m2: {:?}, m3: {:?}, m4: {:?}", m1, m2, m3, m4);

    // 用结构体来实现上面的枚举
    struct QuitMessage; // 单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 元组结构体
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体

    // 实际的案例
    // fn new(stream: TcpStream) {
    //     let mut s = stream;
    //     if tls {
    //         s = negotiate_tls(stream)
    //     }

    // websocket是一个WebSocket<TcpStream>或者
    //   WebSocket<native_tls::TlsStream<TcpStream>>类型
    //   websocket = WebSocket::from_raw_socket(
    // s, ......)
    // }
    // 此时，枚举类型就能帮上大忙：
    // enum Websocket {
    //     Tcp(Websocket<TcpStream>),
    //     Tls(Websocket<native_tls::TlsStream<TcpStream>>),
    // }

    // Option 枚举用于处理空值, rust中没有null ，空值计算依赖option

    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: std::prelude::v1::Option<i32> = None;

    println!(
        "some_number: {:?}, some_string: {:?}, absent_number: {:?}",
        some_number, some_string, absent_number
    );

    let five: std::prelude::v1::Option<i32> = Some(5);
    let six: std::prelude::v1::Option<i32> = plus_one(five);
    let none: std::prelude::v1::Option<i32> = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}

fn print_suit(suit: PokerSuit) {
    println!("Suit: {:?}", suit);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
