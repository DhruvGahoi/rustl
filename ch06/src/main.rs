use std::net::IpAddr;

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddrKind::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddrKind::V6(String::from("::1"));

    // let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1");
    // };

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}
