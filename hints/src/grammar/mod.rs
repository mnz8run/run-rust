/**
`Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),` 这个 `@` 是什么语法

在 Rust 中，`@` 符号在模式匹配（pattern matching）中被称为绑定运算符（binding operator）。它允许你在匹配模式的同时将匹配的值绑定到一个新的变量上。

让我用一个简单的例子来解释：

```rust
// 原始写法
Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}")

// 等价于以下两种写法：

// 1. 不使用 @ 的写法，但需要重复使用整个模式
Err(ExtractError::JsonError { .. } as err) => panic!("{:?}", ExtractError::JsonError { .. })

// 2. 使用 @ 可以更简洁地引用匹配的值
```

在你给出的例子中：

- `err @ ExtractError::JsonError { .. }` 表示：
  1. 匹配 `ExtractError::JsonError` 类型的错误
  2. 同时将整个匹配到的值绑定到变量 `err` 上
  3. `..` 表示忽略该结构体中的其他字段

这种语法特别有用的场景是：

1. 当你需要在匹配某个具体模式的同时，还需要使用整个值的时候
2. 避免重复书写复杂的模式
3. 使代码更简洁易读

这里有一个更完整的例子来说明 `@` 的用法：

```rust
Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32)
}

fn main() {
    let color = Color::RGB(10, 20, 30);

    match color {
        // 使用 @ 将整个 RGB 值绑定到 rgb 变量
        rgb @ Color::RGB(_, _, _) => println!("Got RGB color: {:?}", rgb),

        // 不使用 @，就无法直接引用整个值
        Color::HSV(h, s, v) => println!("Got HSV color: {}, {}, {}", h, s, v),
    }
}
```

在实际开发中，`@` 模式在错误处理、复杂数据结构匹配等场景下特别有用，它可以帮助你写出更简洁、更易维护的代码。

 */

#[derive(Debug)]
enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

fn main() {
    let color = Color::RGB(10, 20, 30);

    match color {
        // 使用 @ 将整个 RGB 值绑定到 rgb 变量
        rgb @ Color::RGB(_, _, _) => println!("Got RGB color: {:?}", rgb),

        // 不使用 @，就无法直接引用整个值
        Color::HSV(h, s, v) => println!("Got HSV color: {}, {}, {}", h, s, v),
    }
}
