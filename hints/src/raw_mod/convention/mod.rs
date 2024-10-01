//! rust 模块根据路径来确定唯一性
//!
//! ## 另一种方式：不使用 mod.rs 在与文件夹同级，创建与文件夹同名文件。
//! src/foo.rs
//! src/foo/mod.rs
//!
//! 如果两者都存在: Rust 将优先使用 src/foo.rs, src/foo/mod.rs 中的内容将被忽略，同时提示错误。
//! ```console
//! file for module `convention` found at both "hints\src\raw_mod\convention.rs" and "hints\src\raw_mod\convention\mod.rs"
//! delete or rename one of them to remove the ambiguity
//! ```
//!
//! 文件要使用下划线命名，中划线命名文件会无法引入
//!
//! rust 2018 开始取消的一个限制，跟子模块有关
//! https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#no-more-modrs

pub mod child;
pub mod convention;
pub const NAME: &str = "convention mod";
