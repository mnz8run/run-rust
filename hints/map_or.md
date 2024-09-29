[细说 Rust 错误处理](https://rustcc.cn/article?id=75dbd87c-df1c-4000-a243-46afc8513074)
[Rust 中的错误处理](https://rustcc.cn/article?id=1e20f814-c7d5-4aca-bb67-45dcfb65d9f9)

Result Error From Option Some

Ok Err Some None

## unwrap except match ? map_err

map_or 解释

###### ChatGPT 说：

ChatGPT

在 Rust 中，`map_or` 是 `Option` 和 `Result` 类型的一个非常有用的实用方法，它允许你在处理可能为 `None` 的 `Option` 或 `Err` 的 `Result` 时，提供一个默认值。如果 `Option` 是 `None` 或 `Result` 是 `Err`，则返回默认值；如果是 `Some` 或 `Ok`，则对值应用一个函数并返回结果。

### `Option::map_or`

对于 `Option<T>` 类型，`map_or` 方法的签名如下：

```rs
impl<T> Option<T> {
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    { ... }
}
```

- `default: U` 是一个当 `Option` 为 `None` 时返回的默认值。
- `f: F` 是一个函数，如果 `Option` 是 `Some(T)`，它将对存储的值 `T` 应用该函数并返回结果。
- 这个方法最终返回类型 `U`，它可以与 `T` 类型不同。

### 示例

假设我们有一个 `Option<i32>`：

```rs
let x: Option<i32> = Some(5);
let y = x.map_or(0, |v| v + 1); // 如果是 Some(5)，返回 6；如果是 None，返回 0
println!("{}", y); // 输出 6
```

在这个例子中，`x` 是 `Some(5)`，所以我们应用了闭包 `|v| v + 1`，结果是 `6`。如果 `x` 是 `None`，则会直接返回 `0`，不会调用闭包。

再来看一个 `None` 的例子：

```rs
let x: Option<i32> = None;
let y = x.map_or(0, |v| v + 1); // 因为是 None，直接返回 0
println!("{}", y); // 输出 0
```

这里，因为 `x` 是 `None`，所以我们跳过了闭包，直接返回默认值 `0`。

### `Result::map_or`

对于 `Result<T, E>`，它的用法和 `Option` 类似，但仅处理 `Ok(T)` 的值。它的签名如下：

```rs
impl<T, E> Result<T, E> {
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    { ... }
}
```

它的工作方式类似，如果 `Result` 是 `Ok(T)`，则对 `T` 应用函数 `f`，否则直接返回 `default`。

### 示例

```rs
let x: Result<i32, &str> = Ok(5);
let y = x.map_or(0, |v| v + 1); // 如果是 Ok(5)，返回 6；如果是 Err，返回 0
println!("{}", y); // 输出 6

let z: Result<i32, &str> = Err("error");
let w = z.map_or(0, |v| v + 1); // 因为是 Err，直接返回 0
println!("{}", w); // 输出 0
```

### 总结

- `map_or` 是用于处理 `Option` 和 `Result` 类型的快捷方法。
- 它允许你在 `None` 或 `Err` 时返回默认值，在 `Some(T)` 或 `Ok(T)` 时对 `T` 应用一个函数。
- 这种方法简化了在常见情况下的错误处理和默认值提供，是一种高效的模式。

如果你有任何具体的场景想讨论，或者还需要更复杂的例子解释，欢迎告诉我！

4o
