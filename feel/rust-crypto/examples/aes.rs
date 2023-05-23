use base64::{engine::general_purpose, Engine as _};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use std::str;

pub fn aes_cbc_mode() {
    let kk = "kk";
    let message = "aa";

    let mut md5er = Md5::new();
    let string = String::from(kk);
    md5er.input_str(&string);
    let md5 = md5er.result_str();
    println!("md5 {}", md5);

    let md5_u8_array = md5.as_bytes();
    println!("md5_u8_array {:?}", md5_u8_array);

    let aes_key: [u8; 32] = md5_u8_array.try_into().unwrap();
    let aes_iv: [u8; 16] = (&aes_key[0..16]).try_into().unwrap();

    // aes_key -> &aes_key : [u8] -> &[u8]
    // std::str::from_utf8(&aes_key).unwrap() -> &str
    // std::str::from_utf8(&aes_key).unwrap().to_string() -> String
    // let ceshi = std::str::from_utf8(&aes_key).unwrap().to_string();

    // String::from_utf8((&aes_key).to_vec()).unwrap()
    // let ceshi = String::from_utf8((&aes_key).to_vec()).unwrap();

    let encrypted_data = aes256_cbc_encrypt(message.as_bytes(), &aes_key, &aes_iv).ok().unwrap();

    let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted_data);
    println!("encrypted_base64 {}", encrypted_base64);
    // Utf8Error
    // let encrypted_message = str::from_utf8(encrypted_data.as_slice()).unwrap();
    // println!("encrypted_message {}", encrypted_message);

    let decrypted_data = aes256_cbc_decrypt(&encrypted_data[..], &aes_key, &aes_iv).ok().unwrap();

    let crypt_message = str::from_utf8(decrypted_data.as_slice()).unwrap();

    assert_eq!(message, crypt_message);
    println!("crypt_message {}", crypt_message);
}

// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
// keywords: aes cbc 256 pkcs7 utf-8 base64
fn aes256_cbc_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn main() {
    aes_cbc_mode();
}
