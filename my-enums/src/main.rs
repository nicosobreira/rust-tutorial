enum IpAddrKind {
    V4(u8, u8, u8),
    V6(String),
    V8(String),
}

impl IpAddrKind {
    fn call(&self) {
        match self {
            IpAddrKind::V4(a, b, c) => println!("{}.{}.{}", a, b, c),
            IpAddrKind::V6(s) => println!("{}", s),
            IpAddrKind::V8(s) => println!(":{}:", s),
        }
    }
}

fn main() {
    let a: [IpAddrKind; 3] = [
        IpAddrKind::V4(127, 0, 0),
        IpAddrKind::V6(String::from("Um texto grande e inválido")),
        IpAddrKind::V8(String::from("123")),
    ];

    for item in &a {
        item.call();

        if let IpAddrKind::V8(_) = item {
            println!("\tExperimental feature");
        }
    }
}
