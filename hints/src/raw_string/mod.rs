fn main() {
    // str Rust 的核心语言中唯一一种字符串类型，通常以其借用形式 &str 形式出现
    // str string slice
    // &str string slice 的借用形式
    // String 由 Rust 的标准库提供

    // String 是一种可增长的、可变的、自有的、UTF-8 编码的字符串类型。
    // 许多可用于 Vec<T> 的相同操作也可用于 String，
    // 因为 String 实际上是作为字节向量的包装器实现的，具有一些额外的保证、限制和功能。

    // 创建 String
    let string = String::new();
    println!("{string}");

    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");

    // 更新 push_str push

    // push_str 添加 str 类型
    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("s4 is {s4}");

    // push 添加 char 类型
    let mut s5 = String::from("lo");
    s5.push('l');
    println!("s5 is {s5}");

    // 使用 + 运算符或 format! 宏的连接字符串
    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; // 注意 s6 已移至此处，不能再使用
    println!("{s8}");

    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    let s13 = format!("{s9}-{s10}-{s11}");
    let s12 = s9 + "-" + &s10 + "-" + &s11;
    println!("{s12}");
    println!("{s13}");

    // Rust 不支持索引访问
    // 一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值。

    // 字节 标量值 字素簇
    // Rust 提供了不同的方式来解释计算机存储的原始字符串数据，
    // 这样每个程序都可以选择它需要的解释，而不管数据使用的是哪种人类语言。

    // 索引操作应该总是花费常数时间 (O(1))。
    // 但是使用 String 无法保证这种性能，因为 Rust 必须遍历从头到索引的内容以确定有多少个有效字符。

    // Slicing Strings
    let hello = "Здравствуйте";
    let s14 = &hello[0..4];
    println!("{s14}");
    // s14 是一个 &str，它包含字符串 hello 的头四个字节
    // &hello[0..1] 运行时会 panic，就跟访问 vector 中的无效索引时一样
    // 你应该小心谨慎地使用这个操作，因为这么做可能会使你的程序崩溃。

    // 遍历字符串的方法
    // 操作字符串每一部分的最好的方法是明确表示需要字符还是字节。

    // 对于单独的 Unicode 标量值使用 chars 方法。
    for c in "Зд".chars() {
        println!("{c}");
    }

    // bytes 方法返回每一个原始字节
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // 标准库并没有提供从字符串中获取如字形簇功能
    // crates.io 上有些提供这样功能的 crate。
}
