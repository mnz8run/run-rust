//! 宏: 生成 Rust 代码的 Rust 代码
//! 与函数的区别：
//!     1. 宏能够接收不同数量的参数；函数签名必须声明函数参数个数和类型。
//!     2. 宏可以在编译器翻译代码前展开；函数是在运行时被调用，同时 trait 需要在编译时实现。
//!     3. 在一个文件里调用宏 之前 必须定义它，或将其引入作用域；而函数则可以在任何地方定义和调用。
//! 声明宏 (Declarative)
//! 过程宏 (Procedural)
//!     自定义宏 (#[derive])
//!     类属性宏 (Attribute-like)
//!     类函数宏
