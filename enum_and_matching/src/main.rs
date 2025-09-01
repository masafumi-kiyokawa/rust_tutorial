fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));
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