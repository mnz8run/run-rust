use rand::Rng;
use std::io; // trait

fn main() {
    println!("猜数游戏");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字 {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取错误");
    println!("你猜的数字是 {}", guess);
}
