//! mod 作用：封装代码
//!
//! mod
//! pub mod
//! use
//! pub use: Re-exporting
//!
//! 文件的方式，不是很推荐

pub mod convention;

// 文件方式引入模块
#[path = "./file/underscore_separation.rs"]
pub mod underscore_separation;

// 连字符的情况；不推荐使用
#[path = "./file/hyphen-separation.rs"]
pub mod hyphen_separation;
