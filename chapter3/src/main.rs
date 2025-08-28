fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is: {}", y);
    }
    println!("The valueof y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;

    let t = true;
    let f = false;

    let c = 'z';
    let z = 'Ｚ';
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];

    another_function()
}

fn another_function() {
    println!("Another function.")
}
