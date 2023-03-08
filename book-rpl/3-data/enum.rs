// 只涉及数据部分，方法部分放在 method

// 表示值是一组可能值中的一个的方式

// 创建枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    // 枚举值
    // IpAddrKind::V4 和 IpAddrKind::V6 都是 IpAddrKind 类型
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // 定义的枚举成员的名字,变成了一个构建枚举的实例的函数
    // 构造函数会自动被定义
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}

// 每个成员都存储了不同数量和类型的值
enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
