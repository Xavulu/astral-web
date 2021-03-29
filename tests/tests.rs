use astral_wasm::*;

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