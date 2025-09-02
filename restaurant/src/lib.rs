mod front_of_house;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub use crate::front_of_house::hosting;
// mod front_of_house {
//     // 同じモジュール内からの呼び出しはOK
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        // 公開
        pub toast: String,      // 公開
        seasonal_fruit: String, // 非公開
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,  // デフォルト公開
        Salad, // デフォルト公開
    }
}

// use crate::front_of_house::hosting; // 絶対パス
// use self::front_of_house::hosting; // 相対パス

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute
    front_of_house::hosting::add_to_waitlist(); // relative

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // seasonal_fruitは非公開なので変更できない

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 親モジュールを持ち込み
use std::fmt;
use std::io;

// 別名を付与
// use std::fmt::Result as FmtResult;
// use std::io::Result as IoResult;

// Result型は様々なモジュールで定義されているのでどのモジュールのResultかわかるように親モジュールまでをスコープに持ち込む
// fn function1() -> fmt::Result {
//     // --snip--
//     // （略）
// }

// fn function2() -> io::Result<()> {
//     // --snip--
//     // （略）
// }

// pub use self::front_of_house::hosting; // 再公開
// 再公開しないと外部モジュールからeat_at_restaurant関数(経由でhosting::add_to_waitlist)を呼び出せない
// pub fn eat_at_restaurant() {
//     // ...
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// // 別々にでも取り込める
// use std::cmp::Ordering;
// use std::io;
// // ネストして1行でも書ける
// use std::{cmp::Ordering, io};

// // このような関係性の場合
// use std::io;
// use std::io::Write;
// // このようにかける
// use std::io::{self, Write};

// // 指定モジュール以下の公開要素を全て持ち込む場合
// use std::collections::*;
