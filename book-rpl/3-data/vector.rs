fn main() {
    // Vec<T> vector
    // 相同类型 长度不限

    // 创建 vector
    let vector: Vec<i32> = Vec::new();
    println!("{:?}", vector);

    // vec! 宏
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    // 更新 vector 可变的才可以更新
    let mut v2 = Vec::new();
    v2.push(7);

    // 索引读取
    // 访问不存在的元素时，Rust 会造成 panic
    let third = v1[2];
    println!("{third}");

    // get 方法访问
    // 访问不存在的元素时，不会 panic 而是返回 None
    let does_not_exist = v1.get(100);
    println!("{:?}", does_not_exist);

    // 遍历 vector
    for i in &v1 {
        println!("{i}");
    }

    // 使用枚举来储存多种类型
    // vector 只能储存相同类型的值。
    // 通过枚举可以达到存放不同类型的值。
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
