fn main() {
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{}", derive_max(&number_list));
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("{}", derive_max(&number_list));

    let number_list = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&char_list));
    
}

fn derive_max(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    return largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest
}