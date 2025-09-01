fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremial@example.com");

    let user2 = User { ..user1 };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area2((width1, height1))
    );

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect)
    );

    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let other_rect = Rectangle {
        width: 40,
        height: 30,
    };
    println!("{}", rect.can_hold(&other_rect));

    println!("{:#?}", Rectangle::square(50));
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        (self.width >= other_rect.width) && (self.height >= other_rect.height)
    }
}
// 分割してもいい
impl Rectangle {
    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
