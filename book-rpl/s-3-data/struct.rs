// 只涉及数据部分，方法部分放在 method

// 数据片段的名字和类型，称之为字段(field)
// 结构体将相关字段(key)和数据(value)组合在一起

// 定义结构体
#[derive(Debug)]
struct Struct {
    a: u64,
    b: bool,
    c: f64,
}
fn main() {
    // 创建实例
    let s1 = Struct { a: 7, b: true, c: 7.13 };
    println!("{:?}", s1);
    // 对应字段的使用
    println!("{}", s1.a);
    println!("{}", s1.b);

    // 改变实例字段值
    // 整个实例必须是可变的，Rust 并不允许只将某个字段标记为可变
    let mut s2 = Struct { a: 7, b: true, c: 7.13 };
    s2.b = false;
    println!("{}", s2.b);

    // 字段初始化简写语法(field init shorthand)
    let a = 13;
    let s3 = Struct { a, b: false, c: 7.13 };
    println!("{}", s3.a);

    // 结构体更新语法(struct update syntax)
    // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
    // .. 必须放在最后
    let s4 = Struct { a: 13, ..s3 };
    println!("{}", s4.c);

    // 元组结构体(tuple structs)
    // 没有具体的字段名，只有字段的类型
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    // black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例
    // 定义的每一个结构体有其自己的类型，即使结构体中的字段可能有着相同的类型。
    // 在其他方面，元组结构体实例类似于元组
    // 可以将它们解构为单独的部分，也可以使用 . 后跟索引来访问单独的值，等等。
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);
    // 索引访问
    println!("{}", origin.1);
    // 元组结构体解构
    let Color(_f, s, _t) = black;
    println!("{}", s);

    // 类单元结构体(unit-like structs) 一个没有任何字段的结构体
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    println!("{:?}", subject);
}
