fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The value of s2 is: '{s2}'");

    let mut s3 = String::from("hello");

    let r1 = &s3; // 問題なし
    let r2 = &s3; // 問題なし
    let r3 = &mut s3; // 大問題！

    // change(&mut r3);

    dangle();
}

fn calculate_length(s: &String) -> usize {
    // 関数の引数に参照を取ることを借用という
    s.len()
}

fn change(s: &mut String) {
    // 可変な参照を借用する
    s.push_str(", world!")
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
    // sをそのまま返せばスコープを抜けても所有権がムーブされるだけなので問題なくなる
    // sの参照を返そうとすると実体のsはスコープを抜けると解放されてしまい無効な参照となる
}