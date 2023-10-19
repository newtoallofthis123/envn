//! Warning: This code was taken __directly__ from the docs and ChatGPT
// TODO: Actually understand this code
// for now, all I need is the key and plaintext as the b"string"

use crypto::aes::{self, KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use rand::RngCore;

pub fn generate_aes_key(key_size: usize) -> Vec<u8> {
    let mut key = vec![0u8; key_size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key); 
    key
}

pub fn encrypt(key: &[u8], normal_text: &[u8]) -> Result<Vec<u8>, crypto::symmetriccipher::SymmetricCipherError> {
    let plaintext = add_padding(&mut normal_text.to_vec(), 16);

    // Create an AES encryptor with the given key
    let mut encryptor = aes::ecb_encryptor(KeySize::KeySize128, key, NoPadding);

    // Initialize buffers
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&plaintext);
    let mut buffer = [0; 16];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    let mut ciphertext = Vec::new();

    // Perform encryption
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        ciphertext.extend(write_buffer.take_read_buffer().take_remaining());
        
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(ciphertext)
}

pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, crypto::symmetriccipher::SymmetricCipherError> {
    // Create an AES decryptor with the given key
    let mut decryptor = aes::ecb_decryptor(KeySize::KeySize128, key, NoPadding);

    // Initialize buffers
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(ciphertext);
    let mut buffer = [0; 16];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    let mut plaintext = Vec::new();

    // Perform decryption
    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        plaintext.extend(write_buffer.take_read_buffer().take_remaining());
        
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(plaintext)
}

pub fn add_padding(data: &mut Vec<u8>, block_size: usize) -> Vec<u8> {
    let padding = block_size - (data.len() % block_size);
    for _ in 0..padding {
        data.push(padding as u8);
    }
    data.clone()
}

pub fn depad(data: &mut Vec<u8> ) -> Vec<u8> {
    let padding = data[data.len() - 1] as usize;
    data.truncate(data.len() - padding);
    data.clone()
}
