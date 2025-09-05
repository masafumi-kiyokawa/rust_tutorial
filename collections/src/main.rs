fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // macro
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    {
        let v = vec![1, 2, 3, 4];
    } // ベクタも要素もドロップする

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 
    println!("The third element is {}", third);

    match v.get(2) {
        // Option<&i32>が返る
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6); // 可変借用
    // println!("The first element is: {}", first);  // すでに可変借用されているので不変借用しようとするとエラーになる

    let v = vec![100, 32, 57];
    for elem in &v {
        println!("{}", elem)
    }

    let mut v = vec![100, 32, 57];
    for elem in &mut v {
        *elem += 50; // 可変参照の参照先の値を変更するには*(参照外し演算子)を使用して値に辿り着く必要がある
    }
    for elem in &v {
        println!("{}", elem);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    let mut s = String::new();
    let data = "some str";
    let s = data.to_string();
    let s = "str".to_string();
    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s = s1 + &s2; // s1の所有権はムーブする
    // fn add(self, s: &str) -> String {}
    // &strの引数に&Stringを渡すと、参照外し型強制により&strとして扱われる
    // 具体的には&String[..]に変えるものと考えることができる
    // selfとある通り、所有権はムーブし、s1は有効でなくなるが、s2は有効なまま
    println!("{}", s);
    // println!("{}", s1);
    println!("{}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // format!マクロは引数の所有権を奪わずにStringを返す
    // println!マクロも同様に所有権を奪わない
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    // 文字列はバイトシーケンスなので添字でアクセスしようとするとエラーになる
    let s1 = String::from("hello");
    println!("{}", s1.len());
    // 適切にスライスすると文字列スライスが得られる
    println!("{}", &s1[0..1]);
    // let h = s1[0];
    let s2 = String::from("Здравствуйте");
    println!("{}", s2.len());
    // 以下は正しくスライスしていないので実行時にエラーになる
    // println!("{}", &s2[0..1]);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    hashmap()
}

use std::{collections::HashMap, hash::Hash};

fn hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{}", scores["Blue"]);

    // タプルのベクタをcollectすることでもHashMapを作成できる
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    // collectにより様々なデータ構造にまとめることができるため、作成したい型を注釈する必要がある
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // let s = "My favorite color is ".to_string() + &field_value; // field_valueの所有権はmapにムーブされており、ここでは使えない
    // println!("{}", s);
    // println!("{:?}", map.get(&field_name));  // keyもムーブされている
    println!("{:?}", map.get("Favorite color")); // getだとOptionが返る
    println!("{:?}", map.get("Not exist key")); // getだとOptionが返る
    println!("{:?}", map["Favorite color"]); // 添字だと値がそのまま返る
    // println!("{:?}", map["Not exist key"]);  // 添字でアクセスしkeyが存在しないとパニック

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value2 = String::from("Red");
    map.insert(field_name, field_value2); // 上書き
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_name2 = String::from("Favorite number");
    let field_value3 = String::from("Yellow");
    let field_value4 = String::from("Three");
    map.entry(field_name).or_insert(field_value3);
    map.entry(field_name2).or_insert(field_value4);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // or_insertはvalueへの可変参照を返すので参照外しをしてcountの値を変更できる
    }
    println!("{:?}", map);

    // training1();

    // training2();
    training3();
}

use std::io;

fn training1() {
    loop {
        println!("Enter some list of numbers!");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| match s.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => panic!("数字をスペース区切りで入力してね"),
            })
            .collect();

        nums.sort();

        println!("nums: {:?}", nums);

        let sum = nums.iter().sum::<i32>() as f64;
        let len = nums.len() as f64;
        println!("mean: {}", sum / len);

        let len = nums.len();
        let median = match len % 2 {
            0 => {
                let prev = nums[(len / 2) - 1] as f64;
                let latt = nums[len / 2] as f64;
                (prev + latt) / 2.0
            }
            1 => nums[len / 2] as f64,
            _ => todo!(),
        } as f32;
        println!("median: {}", median);

        let mut map = HashMap::<i32, usize>::new();
        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        if let Some(max_v) = map.iter().max() {
            let mut mode: Vec<&i32> = map
                .iter()
                .filter_map(|(k, v)| if v == max_v.1 { Some(k) } else { None })
                .collect();
            mode.sort();
            println!("mode: {:?}", mode);
        };
    }
}

fn training2() {
    loop {
        println!("Enter some english word!");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let vowel = vec!['a', 'i', 'u', 'e', 'o'];
        let mut pig_latin = String::new();
        let line = line.lines().collect::<String>();
        let mut chars = line.as_str().chars();
        let first = match chars.next() {
            Some(char) => char,
            None => panic!("invalid string"),
        };
        if vowel.contains(&first) {
            pig_latin = format!("{}-hay", line);
        } else {
            pig_latin = format!("{}-{}ay", chars.as_str(), first);
        }
        println!("{}", pig_latin);
    }
}

fn training3() {
    let mut map = HashMap::<String, Vec<String>>::new();
    loop {
        println!("Enter a operation!");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 4 && words[0] == "Add" && words[2] == "to" {
            let employee = words[1].to_string();
            let department = words[3].to_string();
            map.entry(department).or_insert(Vec::new()).push(employee);
        }
        let mut vec: Vec<(&String, &Vec<String>)> = map.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        let mut all_employees: Vec<&String> = map.values().flatten().collect();
        all_employees.sort();

        println!("{:?}", vec);
        println!("{:?}", all_employees);
    }
}
