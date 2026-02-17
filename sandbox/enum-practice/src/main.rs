#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}, {:#?}", home, loopback);
    println!("{:#?}, {:#?}", home.kind, loopback.address);

    let some_number = Some(5);
    let some_char = Some("e");

    let absent_number: Option<i32> = None;

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    println!("{:?},{:?},{:?}", some_number, some_char, absent_number);
}
