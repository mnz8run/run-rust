fn main() {
    // rust 在编译时就必须知道所有变量的类型
    // 编译器通常可以推断出类型。当多种类型均有可能时，必须增加类型注解
    let _guess: u32 = "42".parse().expect("Not a number!");

    // 标量（scalar）类型代表一个单独的值。
    // 四种基本的标量类型：整型、浮点型、布尔类型、字符类型
    // ======================== 整型 ========================
    // u/i[8,16,32,64,128] usize,isize
    // 不同形式编写数字字面值
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    // Byte (单字节字符)(仅限于u8)
    let byte = b'A';

    println!("{decimal}");
    println!("{hex}");
    println!("{octal}");
    println!("{binary}");
    println!("{byte}");
    // 整数除法会向下舍入到最接近的整数
    let truncated = -5 / 3; // -1
    println!("{truncated}");
    // 整型溢出 two’s complement wrapping

    // ======================== 浮点型 ========================
    // f64 f32 默认 f64
    let f64 = 2.0;
    let f32: f32 = 3.0;
    println!("{f64}");
    println!("{f32}");

    // ======================== 布尔类型 ========================
    let t = true;
    let f: bool = false;
    println!("{t}");
    println!("{f}");
    // ======================== 字符类型 ========================
    // 单引号声明 char 字面量，双引号声明字符串字面量
    // Rust 的 char 类型的大小为四个字节 (four bytes)，并代表了一个 Unicode 标量值(Unicode Scalar Value)
    // Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值
    let char = 'z';
    let z_char: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{char}");
    println!("{z_char}");
    println!("{heart_eyed_cat}");
}
