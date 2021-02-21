
/*
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/


enum IpAddr {
    V4(u8, u8, u8, u8), // you can split up associated enum values 
    V6(String,) //has associated String value
    // you can put any kind of data strings, numeric types, structs, or even enums
}

enum Message { // example; doesn't use struct keyword as all variants are grouped under Message type; structs would all have their own types and harder to define function for any kinds of messages
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { // we can also use impl for enums
    fn call(&self) {
        // method body would be defined here
    }
}

//fn route(ip_kind: IpAddrKind) {}

fn main() {
    
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind:IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
*/

    // this is the short cut instead of creating a struct
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    

}
