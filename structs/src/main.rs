#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 所有権は不要で、構造体のデータを読み込みたいだけ
    // 引数をselfとして、所有権を奪うメソッドを定義することは稀である
    // もとの構造体を更新したい場合には、*mut selfを使用する
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle{ width: 30, height: 50 };
    let rect2 = Rectangle{ width: 10, height: 40 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}
