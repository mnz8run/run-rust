// 文件方式引入模块
#[path = "files/from-file.rs"]
mod from_file;

// from-mod 中划线命名文件无法引入使用。
mod from_mod;

fn main() {
    println!("{}", from_file::WAY);
    println!("{}", from_mod::WAY);
    println!("{}", from_mod::from_mod_child::WAY);
    println!("{}", from_mod::from_mod::WAY);
}
