// The Above file is used to handle the encryption and decryption of the data.
// This works by employing the aes_gcm crate, which is a Rust implementation of the AES-GCM encryption algorithm.
// The AES-GCM algorithm is a symmetric encryption algorithm, which means that the same key is used to encrypt and decrypt the data.
// This might be a bit less secure for web applications, but for a CLI application, it is a good enough solution.
// The encryption and decryption require two things: a key and a nonce.
// A nonce is a number that is used only once, and it is used to prevent replay attacks.
// The nonce is usually not kept secret, and it is sent along with the encrypted data.
// The key, on the other hand, is kept secret and is used to encrypt and decrypt the data.
// In our case, the key is generated using the OsRng, which is a cryptographically secure random number generator.
// Hence, this is a relatively secure way of generating a key.
// The implementation is a bit messy, but it is not too complicated.

use aes_gcm::{
    aead::{Aead, Nonce, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit,
};

/// Generating the unique and cryptographically secure key
pub fn get_key() -> Key<Aes256Gcm> {
    Aes256Gcm::generate_key(OsRng)
}

/// Generating the unique and cryptographically secure nonce
pub fn get_nonce() -> Nonce<Aes256Gcm> {
    Aes256Gcm::generate_nonce(OsRng)
}

/// Encrypts the data using the key and nonce
/// The key and nonce are generated using the get_key and get_nonce functions.
///
/// ## Arguments
///
/// * `key` - The encryption key.
/// * `nonce` - The nonce value.
/// * `data` - The data to be encrypted.
///
/// ## Returns
///
/// The encrypted data as a vector of bytes.
pub fn encrypt(key: Key<Aes256Gcm>, nonce: Nonce<Aes256Gcm>, data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&key);

    cipher.encrypt(&nonce, data.as_ref()).unwrap()
}


/// Decrypts the given data using the specified key and nonce.
///
/// ## Arguments
///
/// * `key` - The encryption key to use for decryption.
/// * `nonce` - The nonce value to use for decryption.
/// * `data` - The data to be decrypted.
///
/// ## Returns
///
/// The decrypted data as a vector of bytes.
pub fn decrypt(key: Key<Aes256Gcm>, nonce: Nonce<Aes256Gcm>, data: Vec<u8>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce.as_ref());

    cipher.decrypt(nonce, data.as_ref()).unwrap()
}
