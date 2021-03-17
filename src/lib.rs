mod panic_hook;

use wasm_bindgen::prelude::*;
use secrecy::{Secret}; 
use brotli::{enc::BrotliEncoderInitParams, BrotliCompress, BrotliDecompress};
use std::{io::{Read, Write}, vec};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT; 

#[wasm_bindgen]
pub fn b3_checksum(in_put: &[u8], out_put: &mut [u8]){
    let mut b3_hasher = blake3::Hasher::new();
    b3_hasher.update(in_put);
    let mut b3_reader = b3_hasher.finalize_xof(); 
    b3_reader.fill(out_put);
} 

#[wasm_bindgen]
pub fn b3_checksum_verify(in_put: &[u8], out_put: &mut [u8], hash: &mut [u8]) -> bool {
    let mut b3_hasher = blake3::Hasher::new();
    b3_hasher.update(in_put);
    let mut b3_reader = b3_hasher.finalize_xof(); 
    b3_reader.fill(out_put);
    
    out_put == hash
}

fn encryption_error<T> (_: T) -> JsValue {
    js_sys::Error::new("failed to encrypt data :(").into()
}

fn decryption_error<T> (_: T) -> JsValue {
    js_sys::Error::new("failed to decrypt data :(").into()
} 

fn compression_error<T> (_: T) -> JsValue {
    js_sys::Error::new("failed to compress data :(").into()
}

fn decompression_error<T> (_: T) -> JsValue {
    js_sys::Error::new("failed to decompress data :(").into()
} 

#[wasm_bindgen]
pub fn encrypt_data(in_put: &[u8], pass: &str, armor: bool) -> Result<Box<[u8]>, JsValue>{
    let mut encrypted = vec![];
    let encryption = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned())); 
    let mode = if armor {
        age::armor::Format::AsciiArmor
    } else {
        age::armor::Format::Binary
    }; 
    let armor = age::armor::ArmoredWriter::wrap_output(&mut encrypted, mode)
        .map_err(encryption_error)?; 
    let mut writer = encryption.wrap_output(armor)
        .map_err(encryption_error)?;
    writer.write_all(in_put).map_err(encryption_error)?; 
    writer
        .finish() 
        .and_then(|armor| armor.finish())
        .map_err(encryption_error)?; 
    Ok(encrypted.into_boxed_slice())
} 

#[wasm_bindgen]
pub fn decrypt_data(in_put: &[u8], pass: &str) -> Result<Box<[u8]>, JsValue>{
    let armor = age::armor::ArmoredReader::new(in_put); 
    let decryption = match age::Decryptor::new(armor)
        .map_err(decryption_error)? {
            age::Decryptor::Passphrase(p) => p, 
            _ => return Err(decryption_error(())),
        }; 
    let mut decrypted = vec![]; 
    let mut reader = decryption
        .decrypt(&Secret::new(pass.to_owned()), None)
        .map_err(decryption_error)?; 
    reader.read_to_end(&mut decrypted).map_err(decryption_error)?; 
    Ok(decrypted.into_boxed_slice())
} 

#[wasm_bindgen]
pub fn compress_data(in_put: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let mut output = vec![];
    let brotli_params = BrotliEncoderInitParams(); 
    match BrotliCompress(&mut &*in_put, &mut output, &brotli_params) {
        Ok(_) => Ok(output), 
        Err(e) => Err(compression_error(e)),
    } 
} 

#[wasm_bindgen]
pub fn decompress_data(in_put: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let mut output = vec![]; 
    match BrotliDecompress(&mut &*in_put, &mut output){
        Ok(_) => Ok(output), 
        Err(e) => Err(decompression_error(e)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn hash_test(){
        let message = b"hello world"; 
        let mut hashed = [0; 32]; //blake3 is a 256 bit hash so 32 bytes
        b3_checksum(message,  &mut hashed); 
        print!("\nmessage: '{}' hash: {:?}\n", str::from_utf8(message).unwrap(), hashed); 
        let mut hash2 = [0; 32]; 
        let verify = b3_checksum_verify(message, &mut hash2, &mut hashed); 
        assert_eq!(verify, true);
    }
    #[test]
    fn encrypt_decrypt_test(){
        let password = "helloworld"; //this should NEVER be a password..... 
        let data = "Tomorrow when the farm boys find this
        freak of nature, they will wrap his body
        in newspaper and carry him to the museum.
        
        But tonight he is alive and in the north
        field with his mother. It is a perfect
        summer evening: the moon rising over
        the orchard, the wind in the grass. And
        as he stares into the sky, there are
        twice as many stars as usual. - Two Headed Baby Calf, Laura Gilpin";  
        let data_bytes = data.as_bytes(); 
        let encrypted = encrypt_data(data_bytes, password, true).unwrap(); 
        let decrypted = decrypt_data(&encrypted, password).unwrap(); 
        assert_eq!(data, str::from_utf8(&decrypted).unwrap());
    } 
    #[test]
    fn compression_test(){
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]; 
        let compressed =  compress_data(test_data).unwrap(); 
        let test_data2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]; 
        assert_eq!(decompress_data(compressed).unwrap(), test_data2); 
    }
}