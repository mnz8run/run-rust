// ! AES256 CBC、CTR mode encrypt decrypt demo
use crypto::aessafe::*;
use crypto::blockmodes::*;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::symmetriccipher::*;
use crypto::{buffer, symmetriccipher};
use std::str;

fn main() {
    aes_ctr_mode();
}

pub fn aes_ctr_mode() {
    let message = "Hello World! AES CTR MODE.";

    let key: [u8; 32] = [0; 32];
    let iv: [u8; 16] = [0; 16];

    let encrypted_data = aes256_ctr_encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let decrypted_data = aes256_ctr_decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();

    let crypt_message = str::from_utf8(decrypted_data.as_slice()).unwrap();

    assert_eq!(message, crypt_message);
    println!("{}", crypt_message);
}

fn aes256_ctr_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    let mut encoder = CtrMode::new(AesSafe256Encryptor::new(key), iv.to_vec());
    encoder.encrypt(&mut read_buffer, &mut write_buffer, true)?;

    final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());
    Ok(final_result)
}

fn aes256_ctr_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    let mut decoder = CtrMode::new(AesSafe256Encryptor::new(key), iv.to_vec());
    decoder.decrypt(&mut read_buffer, &mut write_buffer, true)?;

    final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());
    Ok(final_result)
}
