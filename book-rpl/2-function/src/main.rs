fn main() {
    // Rust 代码中的函数和变量名使用 snake case 规范风格

    // 创建一个函数 fn
    fn function() {
        println!("Another function.");
    }
    function();

    // Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行
    f1();
    fn f1() {
        println!("f1")
    }

    // 在函数签名中，必须声明每个参数的类型
    // 参数(parameters)是特殊变量，是函数签名的一部分
    // 当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）
    // 技术上讲，这些具体值被称为参数(arguments)
    // 在日常交流中，人们倾向于不区分使用 parameter 和 argument 来表示函数定义中的变量或调用函数时传入的具体值
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }
    print_labeled_measurement(5, 'h');

    // 语句(Statements)是执行一些操作但不返回值的指令。
    // 表达式(Expressions)计算并产生一个值。
    // 函数体由一系列的语句和一个可选的结尾表达式构成。
    // 函数定义也是语句，main 函数本身就是一个语句。

    // 语句不返回值
    // let x = (let y = 6); // error
    // rust 不能这么写 x = y = 6

    // 表达式可以是语句的一部分
    // 语句 let y = 6; 中的 6 是一个表达式，它计算出的值是 6。
    // 函数调用是一个表达式。
    // 宏调用是一个表达式。
    // 用大括号创建的一个新的块作用域也是一个表达式

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    // 表达式：
    // {
    //     let x = 3;
    //     x + 1
    // }
    // 是一个代码块，它的值是 4。
    // 这个值作为 let 语句的一部分被绑定到 y 上。
    // 注意 x+1 这一行在结尾没有分号，与你见过的大部分代码行不同。
    // 表达式的结尾没有分号。
    // 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
    // 在接下来探索具有返回值的函数和表达式时要谨记这一点。

    // 具有返回值的函数
    // -> 后声明它的类型
    // 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
    // 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。
    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is: {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    let z = plus_one(5);
    println!("The value of z is: {z}");
}
