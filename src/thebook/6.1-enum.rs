#[derive(Debug)]
struct Ipv6Addr {
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(Ipv6Addr),
    V7(u32, u32, u32),
    V8,
    V9 { width: u32, height: u32 },
}

impl IpAddrKind {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let home_v6 = IpAddrKind::V6(Ipv6Addr {
        address: String::from("::1"),
    });

    let home_v9 = IpAddrKind::V9 {
        width: 10,
        height: 20,
    };

    home.call(); // print self in call method

    println!("{:?}", home_v9);
}
