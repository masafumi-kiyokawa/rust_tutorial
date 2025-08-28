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

    another_function();

    // main_chap3_5();

    continue_example();

    while_example();

    forloop_example();
}

fn another_function() {
    println!("Another function.")
}

fn main_chap3_5() {
    let number = 0;
    if number < 5 {
        println!("condition was true"); // 真の場合
    } else {
        println!("condition was false"); // 偽の場合
    }

    // if number {
    //     println!("number was three") // エラーになる
    // }

    if number != 0 {
        println!("number was something other than zero");
    } else if number == 0 {
        println!("number was zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "five" }; // エラーになる

    println!("The value of number is: {}", number);

    loop {
        println!("In a loop!");
    }
}

fn continue_example() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaning = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn while_example() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!!");
}

fn forloop_example() {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }

    for element in a {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!")
}