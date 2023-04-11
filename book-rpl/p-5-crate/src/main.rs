fn main() {
    // crate 是 Rust 在编译时最小的代码单位。
    // crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译

    // crate 有两种形式：二进制项和库。
    //     二进制项 可以被编译为可执行程序，比如一个命令行程序或者一个服务器。
    //     它们必须有一个 main 函数来定义当程序被执行的时候所需要做的事情。
    //     目前我们所创建的 crate 都是二进制项。

    //     库 并没有 main 函数，它们也不会编译为可执行程序，
    //     它们提供一些诸如函数之类的东西，使其他项目也能使用这些东西。

    // 大多数时间 Rustaceans 说的 crate 指的都是库，这与其他编程语言中 library 概念一致。

    // crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块。

    // Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。
    // 同样的，Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。
    // crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

    // 如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。
    // 通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。

    println!("Hello, world!");
}
