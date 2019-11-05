fn main() {
    // Stringを利用すると、ヒープ上にメモリを格納するため、コンパイル時にサイズがわからないテキストを保持できる
    // let s = "hello"; とした場合には、文字列リテラル"hello"は不変となる
    let mut s = String::from("hello");
    // したがって、次のような操作がStringに対しては可能となる「文字列リテラルの追加」
    s.push_str(", world!");

    // 次のようなコードでは、sに束縛される値の「所有権」がs2にmoveするので
    // コンパイルエラー: "value borrowed here after move" が発生する
    // let s2 = s;

    println!("{}", s); // let s2 = s; println!("{}, s2"); とした場合にはエラーにはならない

    /* ------------------------------------------------ */

    let t = String::from("hello");  // tがスコープに入る

    takes_ownership(t);             // tの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
