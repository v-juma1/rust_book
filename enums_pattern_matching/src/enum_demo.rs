//定义枚举
#[derive(Debug)]
enum IPKind {
    ipv4,
    ipv6,
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
        kind: IPKind::ipv4,
        address: String::from("192.168.89.9"),
    };

    let v6_1 = IPAddr {
        kind: IPKind::ipv6,
        address: String::from("2001:0db8:85a3:08d3:1319:8a2e:0370:7344"),
    };

    println!("{:#?}", v4_1)
}

//定义枚举并绑定数据
//可以绑定不同的数据类型，包括绑定另外的枚举和struct
#[derive(Debug)]
enum IPAddrKind {
    v4(u8, u8, u8, u8),
    v6(String),
}

//为枚举定义方法
impl IPAddrKind {
    fn show(&self) {
        println!("{:#?}", self);
    }
}

//实例化枚举
fn build_IPAddrKind() {
    let v4_1 = IPAddrKind::v4(192, 0, 0, 0);
    let v6_1 = IPAddrKind::v6(String::from("0000:0000:0000:0000:0000:0000:0000:0000"));
    v4_1.show();
    v6_1.show();
}

pub fn run_demo() {
    show_enum();
    build_IPAddrKind();
}
