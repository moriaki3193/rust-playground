// 列挙型は取りうる値をすべて列挙できることが名前の由来

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let mut msg = String::from("hello, ");
    let usr: Option<&str> = Some("moriaki3193");

    match usr {
        Some(s) => msg.push_str(s),
        None => msg.push_str("world!"),
    }

    println!("msg: {}", msg);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };
}
