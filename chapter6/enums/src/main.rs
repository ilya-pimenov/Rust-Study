#[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuiteMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body goes here
    }
}

fn main() {
    println!("Hello, world!");

    //    let four = IpAddrKind::V4;
    //    let six = IpAddrKind::V6;
    //
    //    let home = IpAddr {
    //        kind: IpAddrKind::V4,
    //        address: String::from("127.0.0.1"),
    //    };
    //
    //    let loopback = IpAddr {
    //        kind: IpAddrKind::V6,
    //        address: String::from("::1"),
    //    };

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message:Write(String::from("hello"));
    m.call();

    route(four);

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn route(ip_addr: IpAddrKind) {
    println!("{:?}", ip_addr)
}
