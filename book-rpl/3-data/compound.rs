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
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    // 数组类型 分号前声明类型，分号后声明数量
    let explicit_array: [i32; 5] = [6, 7, 8, 9, 10];
    // 创建多个相同元素数组 分号前是元素，分号后是数量
    let same_array = [567; 13];
    let same_char_array = ['o'; 7];
    println!("{:?}", explicit_array);
    println!("{:?}", same_array);
    println!("{:?}", same_char_array);

    // 解构数组
    let [_a, _b, c, _d, _e] = array;
    println!("{c}");

    // 索引访问
    let first_char = same_char_array[0];
    println!("{first_char}");
}
