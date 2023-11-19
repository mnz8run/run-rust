// use momoa::ast::*;
use momoa::json;
use std::fs;
use std::path::Path;

fn main() {
    // momoa rust 版还有很多要处理的

    // let demo_file_path = Path::new("../../feel/momoa/src/a.json");
    // 路径不一样 启动方式不一样
    let demo_file_path = Path::new("feel/momoa/src/a.json");
    // let demo_file_path = Path::new("feel/pest/json/order.json");
    let unparsed_file = fs::read_to_string(demo_file_path).expect("cannot read file");
    let ast = json::parse(&unparsed_file).unwrap();
    println!("{:?}", ast);

    // do something with ast

    // ast

    let text = "Hello привет";
    let end = text.char_indices().map(|(i, _)| i).nth(8).unwrap();
    println!("{}", &text[2..end]);
}
