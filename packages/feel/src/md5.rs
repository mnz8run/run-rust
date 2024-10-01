use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut hasher = Md5::new();

    let text = String::from("123456");

    hasher.input_str(&text);

    println!("{} => {}", text, hasher.result_str())
}
