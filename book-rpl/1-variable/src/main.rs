fn main() {
    // 常量总是不可变
    // Rust 对常量的命名约定是在单词之间使用全大写加下划线
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 不可变变量
    let x = 5;

    // 可变变量
    let mut y = 5;
    y = y + y;

    // Shadowing 可以改变类型
    let z = 22;
    let z = z + 66;
    let omega = "omega";
    let omega = omega.len();

    println!("{THREE_HOURS_IN_SECONDS}");
    println!("{x}");
    println!("{y}");
    println!("{z}");
    println!("{omega}");
}
