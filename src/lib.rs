mod panic_hook;

use panic_hook::*;
use wasm_bindgen::prelude::*;
use secrecy::{Secret}; 
use brotli::{enc::BrotliEncoderInitParams, BrotliCompress, BrotliDecompress};
use std::{io::{Read, Write}, vec};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT; 

#[wasm_bindgen]
pub fn b3_checksum(in_put: &[u8], out_put: &mut [u8]){
    set_panic();
    let mut b3_hasher = blake3::Hasher::new();
    b3_hasher.update(in_put);
    let mut b3_reader = b3_hasher.finalize_xof(); 
    b3_reader.fill(out_put);
} 

#[wasm_bindgen]
pub fn b3_checksum_verify(in_put: &[u8], out_put: &mut [u8], hash: &mut [u8]) -> bool {
    set_panic();
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
    set_panic();
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
    set_panic();
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
    set_panic();
    let mut output = vec![];
    let brotli_params = BrotliEncoderInitParams(); 
    match BrotliCompress(&mut &*in_put, &mut output, &brotli_params) {
        Ok(_) => Ok(output), 
        Err(e) => Err(compression_error(e)),
    } 
} 

#[wasm_bindgen]
pub fn decompress_data(in_put: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    set_panic();
    let mut output = vec![]; 
    match BrotliDecompress(&mut &*in_put, &mut output){
        Ok(_) => Ok(output), 
        Err(e) => Err(decompression_error(e)),
    }
}


