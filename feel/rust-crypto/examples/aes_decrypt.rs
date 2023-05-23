use base64::{engine::general_purpose, Engine as _};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::{
    aes::{cbc_decryptor, KeySize::KeySize256},
    blockmodes, buffer,
    symmetriccipher::SymmetricCipherError,
};

fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = cbc_decryptor(KeySize256, key, iv, blockmodes::PkcsPadding);

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

fn aes_cbc_mode(message: &str, key: &str) -> String {
    let mut md5er = Md5::new();
    let string = String::from(key);
    md5er.input_str(&string);
    let md5 = md5er.result_str();

    let md5_u8_array = md5.as_bytes();

    let aes_key: [u8; 32] = md5_u8_array.try_into().expect("aes_key there_error");
    let aes_iv: [u8; 16] = (&aes_key[0..16]).try_into().expect("aes_iv there_error");

    // println!("message as_bytes {:?}", message.as_bytes());
    let base64_decode_result = general_purpose::STANDARD.decode(message);
    // println!("base64_decode_result {:?}", base64_decode_result);

    let message_decode_base64 = match base64_decode_result {
        Ok(ok) => ok,
        Err(error) => panic!("base64 decode error: {:?}", error),
    };

    let decrypted_data_result = aes256_cbc_decrypt(&message_decode_base64[..], &aes_key, &aes_iv);

    let decrypted_data = match decrypted_data_result {
        Ok(ok) => ok,
        Err(error) => panic!("aes256_cbc_decrypt error: {:?}", error),
    };

    let crypt_message = String::from_utf8(decrypted_data).expect("crypt_message there_error");

    return crypt_message;
}

fn main() {
    let aes_decrypt = aes_cbc_mode("7iS39GJBqwGD8hKPnJ0ZhQ==", "456");
    println!("aes_decrypt {}", &aes_decrypt);
}
