use crypto::digest::Digest;
// ! AES256 CBC、CTR mode encrypt decrypt demo
use crypto::md5::Md5;
use std::str;
use crypto::{symmetriccipher,buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use crypto::aessafe::*;
use crypto::blockmodes::*;
use crypto::symmetriccipher::*;


pub fn aes_cbc_mode(){
    let kk = "kk";
    let message="Hello World!";

    let mut md5 = Md5::new();

    let mut key:[u8;32]=[0;32];
    let mut iv:[u8;16]=[0;16];

    let a = md5.input_str(&kk);
    println!("{:?}",a);



    let encrypted_data=aes256_cbc_encrypt(message.as_bytes(),&key,&iv).ok().unwrap();
    let decrypted_data=aes256_cbc_decrypt(&encrypted_data[..],&key,&iv).ok().unwrap();

    let crypt_message=str::from_utf8(decrypted_data.as_slice()).unwrap();

    assert_eq!(message,crypt_message);
    println!("{}",crypt_message);
}

// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
    let mut encryptor=aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result=Vec::<u8>::new();
    let mut read_buffer=buffer::RefReadBuffer::new(data);
    let mut buffer=[0;4096];
    let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

    loop{
        let result=try!(encryptor.encrypt(&mut read_buffer,&mut write_buffer,true));

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow=>break,
            BufferResult::BufferOverflow=>{},
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn main() {
    aes_cbc_mode();
}

// pub fn aes_ctr_mode(){
//     let message="Hello World! AES CTR MODE.";

//     let mut key:[u8;32]=[0;32];
//     let mut iv:[u8;16]=[0;16];



//     let encrypted_data=aes256_ctr_encrypt(message.as_bytes(),&key,&iv).ok().unwrap();
//     let decrypted_data=aes256_ctr_decrypt(&encrypted_data[..],&key,&iv).ok().unwrap();

//     let crypt_message=str::from_utf8(decrypted_data.as_slice()).unwrap();

//     assert_eq!(message,crypt_message);
//     println!("{}",crypt_message);
// }

// fn aes256_ctr_encrypt(data: &[u8],key: &[u8],iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
//     let mut final_result=Vec::<u8>::new();
//     let mut read_buffer=buffer::RefReadBuffer::new(data);
//     let mut buffer=[0;4096];
//     let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

//     let mut encoder=CtrMode::new(AesSafe256Encryptor::new(key),iv.to_vec());
//     encoder.encrypt(&mut read_buffer,&mut write_buffer,true)?;

//     final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
//     Ok(final_result)
// }

// fn aes256_ctr_decrypt(encrypted_data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
//     let mut final_result = Vec::<u8>::new();
//     let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
//     let mut buffer = [0; 4096];
//     let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

//     let mut decoder=CtrMode::new(AesSafe256Encryptor::new(key),iv.to_vec());
//     decoder.decrypt(&mut read_buffer,&mut write_buffer,true)?;

//     final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
//     Ok(final_result)
// }
