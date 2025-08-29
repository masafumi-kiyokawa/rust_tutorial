fn main() {
    println!("Hello, world!");

    // 文字列リテラルの例
    {                      // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";   // sは、ここから有効になる

        // sで作業をする
    }                      // このスコープは終わり。もうsは有効ではない
    // 変数sは不変、文字列リテラルは固定長であり、スタックに積まれる
    // 文字列リテラルの場合中身はコンパイル時に判明しているため最終的なバイナリファイルにハードコードされる

    // String型の例
    {
        let mut s = String::from("hello"); // ここでヒープ確保が発生
    } // スコープ終了時に drop() が走り、ヒープ領域を解放
        // s を操作
    // 変数sはmutable、String型は可変長であり、ヒープに積まれる
    // String型はコンパイル時には必要なメモリ量が不明なため、実行時にOSに要求、ヒープに確保し、使い終わったら返還する

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);  // これは無効

    let s1 = String::from("hello");  // 1つ目のヒープデータを作成する。s1は1つ目のヒープデータへのポインタを持つ。
    let s2 = s1.clone(); // 1つ目のヒープデータをコピーし2つ目のヒープデータを作成する。s2は2つ目のヒープデータへのポインタを持つ

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;  // Copy注釈によりxは以前使用可能

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                        // ムーブする

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。