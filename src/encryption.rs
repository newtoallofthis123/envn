use aes_gcm::{Key, Aes256Gcm, KeyInit, aead::{OsRng, Nonce, Aead}, AeadCore};

pub fn get_key()-> Key<Aes256Gcm>{
    let key = Aes256Gcm::generate_key(OsRng);
    return key;
}

pub fn get_none()-> Nonce<Aes256Gcm>{
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    return nonce;
}

pub fn encrypt(key: Key<Aes256Gcm>, nonce: Nonce<Aes256Gcm>, data: &[u8])-> Vec<u8>{
    let cipher = Aes256Gcm::new(&key);

    let ciphertext = cipher.encrypt(&nonce, data.as_ref()).unwrap();
    return ciphertext;
}

pub fn decrypt(key: Key<Aes256Gcm>, nonce: Nonce<Aes256Gcm>, data: Vec<u8>)-> Vec<u8>{ 
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce.as_ref());

    let plaintext = cipher.decrypt(&nonce, data.as_ref()).unwrap();
    return plaintext;
}
