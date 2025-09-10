use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{}", derive_max(&number_list));
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("{}", derive_max(&number_list));

    let number_list = vec![34, 50, 25, 100, 65];
    // println!("{}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    // println!("{}", largest(&char_list));

    let integer_integer = Point { x: 5, y: 10 };
    let float_float = Point { x: 1.0, y: 4.0 };

    let mix = Point { x: 1, y: 4.0 };
    println!("p.x = {}", mix.x());
    println!("p.y = {}", mix.y());

    println!(
        "distance from origin = {}",
        float_float.distance_from_origin()
    );

    let mixed = integer_integer.mixup(float_float);

    println!("mixed.x = {}, mixed.y = {}", mixed.x(), mixed.y());

    let tweet = returns_summarizable();

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from(""),
        location: String::from(""),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in th NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // トレイトを実装する型を受け取るようにすればOK
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn derive_max(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {  // >が全てのTで実装されている必要がある
//             largest = item;
//         }
//     }
//     return largest;
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // >が全てのTで実装されている必要がある
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 単相化により性能はジェネリクスの使用有無により変化しない

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

//　↑は↓の syntax sugar
pub fn notify<T: Summary + Display>(item: &T) {
    // +で複数指定可能
    println!("Breaking news! {}", item.summarize());
}

// 以下の場合、function_name<T: Trait>(arg: &T) {}の方が簡潔
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Symmary>(item1: &T, item2: &T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// ↑の定義は↓のようにwhere句で書ける
// fn some_function<T, U>(t: &T, u: &U)
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    }
    // fn returns_summarizable(switch: bool) -> impl Summary {
    // if switch {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // } else {
    //     NewsArticle {
    //         headline: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // }
    // ↑のように1種類より多くの型を返すことができるようにはできない
}

struct Pair<T> {
    x: T,
    y: T,
}
// 全ての型で呼べる
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// DisplayとPartialOrdを実装している型ならcmp_displayを呼べる
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Displayトレイトを実装する全ての型TにToStringトレイトを実装するブランケット実装
// impl<T: Display> ToString for T {

// }