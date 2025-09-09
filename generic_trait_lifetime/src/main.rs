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
