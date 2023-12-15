- 声明模块 mod

  (1) 从文件路径声明

  ```rust
  #[path = "xxx/yyy.rs"]
  pub mod yyy;
  ```

  (2) 从文件结构声明

  1.首选方式 (the preferred way)

  ```
  src
  tool.rs
  main.rs
  ```

  `mod tool` in main.rs

  ```
  src
  tool
      child.rs
  tool.rs
  main.rs
  ```

  `mod tool` in main.rs

  `mod child` in tool.rs

  2.旧方式 (the old way)

  ```
  src
  tool
      mod.rs
  main.rs
  ```

  `mod tool` in main.rs

  ```
  src
  tool
      mod.rs
      child.rs
  main.rs
  ```

  `mod tool` in main.rs

  `mod child` in tool.rs

  粗略地说, mod.rs 有点像 JavaScript index.js 或者 python `__init__.py`。但只是某种程度。这在 Rust 中有点复杂。
