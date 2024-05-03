/**
 * mod 方式 的模块
 *
 * rust 模块根据 路径 来确定唯一性
 *
 * 1. src/foo.rs
 * 2. src/foo/mod.rs mod.rs
 * 如果两者同时存在: Rust 将优先使用 src/foo.rs, src/foo/mod.rs 中的内容将被忽略
 *
 * src/from_mod.rs             from_mod
 * src/from_mod/from_mod.rs    from_mod::from_mod
 * 是两个不同的路径(path)
 *
 *
 * 文件要使用下划线命名，中划线命名文件会无法引入
 *
 * rust 2018 开始取消的一个限制
 * https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#no-more-modrs
 * 跟子模块有关
 *
 */
pub const WAY: &str = "note/mod/src/from_mod.rs";

// 两个子模块
pub mod from_mod;
pub mod from_mod_child;
