use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    let f = File::open("hello.txt");

    // match
    // Pros: 何をどう扱っているか一目瞭然
    // Cons: 1ケースだけなら冗長
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file {:?}", error)
    //     },
    // };

    // if let
    // Pros: 取りたいケースが一つなら短い
    // Cons: 複数分岐や複雑処理には不向き
    // let f = if let Ok(file) = f {
    //     file
    // } else {
    //     panic!("There was a problem opening the file {:?}", f.err().unwrap());
    // };

    // expect
    // 最短・メッセージ付き
    // 本番で落としたくないなら不適
    // let f = File::open("hello.txt").expect("There was a problem opening the file");

    // unwrap_or_else
    // Pros: ログ・計測・クリーンアップ等が可能
    // Cons: 少し冗長
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     panic!("There was a problem opening the file {:?}", error)
    // });

    // ?
    // fn open_file() -> io::Result<File> {
    //     let f = File::open("hello.txt")?; // エラー処理を呼び出し元に委ねる
    //     Ok(f)
    // }
    // let f = open_file().unwrap_or_else(|err| panic!("There was a problem opening file {:?}", err));

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {  // マッチガード
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e)
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file {:?}", error)
    //     },
    // };

    // expectはメッセージ付き、unwrapはメッセージなし
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = File::open("hello.txt")?;  // main関数は戻り値が()なのでResult<Ok, Err>を返す?は使えない
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // 最後の式なのでreturnは省略できる
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // エラーならErr<io:Error>か返る
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // エラーならErr<io:Error>か返る
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // 連結も可能

    Ok(s)
}

use std::net::IpAddr;

fn ipaddr() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();  // ここでは確実にparseできるのでunwrapしてOkの中身を取り出すのがいい
}

// * 悪い状態がときに起こるとは予想されないとき。
// * この時点以降、この悪い状態にないことを頼りにコードが書かれているとき。
// * 使用している型にこの情報をコード化するいい手段がないとき。
// などにだけpanic!させるのがいい

// 数あてゲームでユーザーの入力を独自の型を使って検査する
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || 100 < value {
            panic!("Guess value must be between 1 to 100. got {}", value)
        }

        Guess {
            value
        }
    }
}


// Resultを返すようにした例
// コンストラクタだからあれだけど
// impl Guess {
//     pub fn new(value: u32) -> Result<Guess, String> {
//         if value < 1 || value > 100 {
//             return Err("Guess value must be between 1 to 100".to_string())
//         }

//         Ok(Guess {
//             value
//         })
//     }
// }