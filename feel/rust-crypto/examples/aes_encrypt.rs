use base64::{engine::general_purpose, Engine as _};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::{
    aes::{cbc_encryptor, KeySize::KeySize256},
    blockmodes, buffer,
    symmetriccipher::SymmetricCipherError,
};

fn aes256_cbc_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = cbc_encryptor(KeySize256, key, iv, blockmodes::PkcsPadding);

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

fn aes_cbc_mode(message: &str, key: &str) -> String {
    let mut md5er = Md5::new();
    let string = String::from(key);
    md5er.input_str(&string);
    let md5 = md5er.result_str();

    let md5_u8_array = md5.as_bytes();

    let aes_key: [u8; 32] = md5_u8_array.try_into().expect("aes_key there_error");
    let aes_iv: [u8; 16] = (&aes_key[0..16]).try_into().expect("aes_iv there_error");

    let encrypted_data = aes256_cbc_encrypt(message.as_bytes(), &aes_key, &aes_iv)
        .ok()
        .expect("encrypted_data there_error");

    let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted_data);

    return encrypted_base64;
}

fn main() {
    let aes_encrypt = aes_cbc_mode("123", "456");
    println!("aes_encrypt {}", &aes_encrypt);
}
