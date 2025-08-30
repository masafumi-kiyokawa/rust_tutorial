fn main() {
    // let s = String::from("Hello, world!");
    let s = "hello world";
    let first_word_of_s = first_word(&s);
    // s.clear();
    println!("{first_word_of_s}");

    let s2 = &s[..5];
    let first_word_of_s2 = first_word(&s2);
    println!("{first_word_of_s2}");

    let a = [1, 2, 3, 4, 5];
    let slice_of_list = &a[1..3];

}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return &s[..i];
        }
    }

    // s.len()
    &s[..]
}