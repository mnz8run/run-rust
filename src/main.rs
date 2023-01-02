use rand::Rng;
use std::cmp::Ordering;
use std::io; // trait

fn main() {
    println!("猜数游戏");
    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64

    loop {
        println!("猜测一个数字");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取错误");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是 {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
