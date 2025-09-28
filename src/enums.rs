// An Enum is a verstalie tool use to represent a value that could be one of several different types
pub fn enums() {
    println!("--- Enums ---");
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("128.1.1.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let _msg = Message::Write(String::from("hello"));
}
