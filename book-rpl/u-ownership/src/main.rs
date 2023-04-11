fn main() {
    // Ownership 是一套管理规则，管理 Rust 程序内存。
    // 内存通过 所有权系统 和 一组编译器检查规则 来管理的。
    // 如果违反任何规则，程序将无法编译。
    // 所有权的任何功能都不会在程序运行时减慢程序的速度。

    // 理解 the stack, the heap
    // 了解所有权后，您将不需要经常考虑堆栈和堆，
    // 但了解所有权的主要目的是管理堆数据有助于解释其工作方式的原因。

    // 所有权规则
    // - Rust 中的每个值都有一个所有者。
    // - 值在任一时刻有且只有一个所有者。
    // - 当所有者超出范围时，该值将被删除。

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // 作用域
}
