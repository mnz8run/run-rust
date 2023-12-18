use std::iter::repeat;

use crypto::digest::Digest;
use crypto::md5::Md5;

use rustc_serialize::hex::ToHex;

fn main() {
    let key = "key";
    let mut md5er = Md5::new();
    md5er.input(key.as_bytes());
    let mut md5_result: Vec<u8> = repeat(0).take((md5er.output_bits() + 7) / 8).collect();
    md5er.result(&mut md5_result);

    println!("Hello, world!");
    println!("Hello, world! {}", md5er.result_str());
    println!("Hello, world! {}", md5_result[..].to_hex());
}
