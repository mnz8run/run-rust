## create subpackage

init or new

`cargo init book-rpl/p-1-macros --name rpl-macros`

`cargo init book-rpl/2-data --name rpl-data`

`cargo new book-rpl/1-variable --name rpl-variable`

adding `book-rpl/1-variable` to the `workspace.members` array of the manifest located at root

## run subpackage

可以到相应目录下 `cargo run`

根目录下 packeage, bin, example 都可以运行

packeage 可以跨目录运行，bin, example 不可以跨目录

eg: feel/pest 下不能运行 `cargo run --bin feel-comfy-table`, `cargo run --example comfy_table`

### package

`cargo run -p feel-comfy-table`

### binary

`cargo run --bin feel-comfy-table`

### example

`cargo run --example comfy_table`

`cargo run --example hyphen-named`

## workspace

workspace.members 字段填写的是包的文件夹路径，不是包名，不是二进制文件名。

The `members` list also supports [globs](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html) to match multiple paths, using typical filename glob patterns like `*` and `?`.

> Cargo will automatically search

## cargo

默认情况下：
**package name 取自 directory name; binary name 取自 package name**
