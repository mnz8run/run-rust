//! Rust 代码中的函数使用 snake case 规范风格

fn main() {
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

    // 我们可以定义具有参数的函数，这些参数是作为函数签名一部分的特殊变量。
    // 当函数拥有参数 (形参: parameter) 时，可以为这些参数提供具体的值 (实参: arguments)
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
    // 是一个代码块，在这种情况下，计算结果为 4。
    // 该值作为 let 语句的一部分绑定到 y。
    // 请注意，x + 1 行末尾没有分号，这与您目前看到的大多数行不同。
    // 表达式不包括结束分号。
    // 如果你在表达式的末尾添加分号，你就把它变成了一个语句，然后它就不会返回值。
    // 在接下来探索函数返回值和表达式时，请记住这一点。

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
