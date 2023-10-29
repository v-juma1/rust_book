#![allow(dead_code)]
#![allow(unused_variables)]

//定义枚举
#[derive(Debug)]
enum IPKind {
    Ipv4,
    Ipv6,
}

//在struct中使用枚举类型
#[derive(Debug)]
struct IPAddr {
    kind: IPKind,
    address: String,
}

fn show_enum() {
    //给枚举赋值并实例化struct
    let v4_1 = IPAddr {
        kind: IPKind::Ipv4,
        address: String::from("111.111.11.1"),
    };

    let v6_1 = IPAddr {
        kind: IPKind::Ipv6,
        address: String::from("2001:2001:2001:2001:2001:2001:2001:2001"),
    };

    println!("{:#?}", v4_1)
}

//定义枚举并绑定数据
//可以绑定不同的数据类型，包括绑定另外的枚举和struct
#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

//为枚举定义方法
impl IPAddrKind {
    fn show(&self) {
        println!("{:#?}", self);
    }
}

//实例化枚举
fn build_ipaddr_kind() {
    let v4_1 = IPAddrKind::V4(192, 0, 0, 0);
    let v6_1 = IPAddrKind::V6(String::from("0000:0000:0000:0000:0000:0000:0000:0000"));
    v4_1.show();
    v6_1.show();
}

pub fn run_demo() {
    show_enum();
    build_ipaddr_kind();
}
