use rand::Rng;
use std::cmp::Ordering;
use std::io; // trait

fn main() {
    println!("猜数游戏，猜测一个数字");
    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64
    println!("神秘数字 {}", secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取错误");

    // shadow
    let guess: u32 = guess.trim().parse().expect("not a number");

    println!("你猜的数字是 {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
