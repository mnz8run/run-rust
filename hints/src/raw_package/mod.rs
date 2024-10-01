//! 包(package)是提供一系列功能的一个或者多个箱(crate)。
//! 一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。
//!
//! 包中可以包含至多一个库 crate(library crate)。
//! 包中可以包含任意多个二进制 crate(binary crate)。
//! 但是必须至少包含一个 crate（无论是库的还是二进制的）。
