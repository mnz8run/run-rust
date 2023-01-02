use std::io;

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取错误");
    println!("你猜的数字是 {}", guess);
}
