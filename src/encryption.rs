use aes_gcm::{
    aead::{Aead, Nonce, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit,
};

pub fn get_key() -> Key<Aes256Gcm> {
    Aes256Gcm::generate_key(OsRng)
}

pub fn get_nonce() -> Nonce<Aes256Gcm> {
    Aes256Gcm::generate_nonce(OsRng)
}

pub fn encrypt(key: Key<Aes256Gcm>, nonce: Nonce<Aes256Gcm>, data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&key);

    cipher.encrypt(&nonce, data.as_ref()).unwrap()
}

pub fn decrypt(key: Key<Aes256Gcm>, nonce: Nonce<Aes256Gcm>, data: Vec<u8>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce.as_ref());

    cipher.decrypt(nonce, data.as_ref()).unwrap()
}
