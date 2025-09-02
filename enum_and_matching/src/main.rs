fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5); // SomeならOption<T>のTを推論できる
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // NoneならOption<T>のTを推論できないので明示する必要がある

    // let sum = some_number + 5;  // Option<i8>とi8は別な型なのでコンパイルできない

    let coin = Coin::Nickel;
    println!("{}", value_in_cents(coin));
    let coin2 = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    let some_u8_value2 = 7;
    match some_u8_value2 {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {}

enum IpAddr {
    V4(String),
    // V4(u8, u8, u8, u8)でも可能
    V6(String),
}

enum Message {
    Quit,                       // 紐付けデータなし
    Move { x: i32, y: i32 },    // 匿名構造体が紐づく
    Write(String),              // 文字列
    ChangeColor(i32, i32, i32), // タプル
}

// 上と同じデータを格納できる
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // メソッド本体はここに定義される
        println!("call!!!!!")
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Lucky nickel!!!!!!");
            5
        }
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Optionが取りうる値を全て扱えるようにする必要がある(_を含む)
        Some(i) => Some(i + 1),
    }
}
