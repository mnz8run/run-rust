fn main() {
    // 复合类型（Compound types）可以将多个值组合成一个类型。
    // 两个原生的复合类型: 元组(tuple) 数组(array)
    // ======================= 元组 =======================
    // 类型不必相同，长度固定，一旦声明不会改变
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    // 模式匹配(pattern matching)来解构(destructure)元组
    let (_x, y, _z) = tup;
    println!("{y}");
    // 索引访问
    let five_hundred = tup.0;
    println!("{five_hundred}");
    // ======================= 数组 =======================
    // 类型必须相同，长度固定
}
