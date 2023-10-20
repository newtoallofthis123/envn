use aes_gcm::{Key, Nonce, Aes256Gcm};

use crate::{file::file_exists, db::Entry};

#[derive(Debug)]
pub struct Env {
    pub name: String,
    pub key: String,
    pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct DisplayEnv{
    pub name: String,
    pub key: String,
    pub value: String,
}

pub fn check_password() -> bool {
    let password = inquire::Password::new("Enter your password 👀")
        .without_confirmation()
        .prompt()
        .expect("Failed to get password");
    let key_file = crate::file::get_path("auth");

    if !file_exists(&key_file){
        return false;
    }

    let hashed = std::fs::read_to_string(key_file).expect("Failed to read key file");

    return bcrypt::verify(password, &hashed).expect("Failed to verify password");
}

pub fn construct_struct(name: String, key: String, value: String)->Env{
    let (user_key, nonce) = crate::file::get_keys_and_nonce();

    let bytes_of_value = bincode::serialize(&value).expect("Failed to serialize value");
    let encrypted_value = crate::encryption::encrypt(Key::<Aes256Gcm>::from_slice(&user_key).to_owned(), Nonce::from_slice(&nonce).to_owned(), &bytes_of_value);
    return Env{
        name,
        key,
        value: encrypted_value,
    }
}

pub fn decrypt_struct(entry: Entry)->DisplayEnv{
    let (user_key, nonce) = crate::file::get_keys_and_nonce();
    let decrypted_value = crate::encryption::decrypt(Key::<Aes256Gcm>::from_slice(&user_key).to_owned(), Nonce::from_slice(&nonce).to_owned(), entry.value);
    let value = bincode::deserialize(&decrypted_value).expect("Failed to deserialize value");
    return DisplayEnv{
        name: entry.name,
        key: entry.key,
        value,
    }
}

pub fn display_env(env: DisplayEnv){
    bunt::println!("{$blue}Showing Secret Secret{/$}");
    bunt::println!("{$yellow}Name{/$}: {$green}{}{/$}", env.name);
    bunt::println!("{$yellow}Key{/$}: {$green}{}{/$}", env.key);
    bunt::println!("{$yellow}Value{/$}: {$green}{}{/$}", env.value);
}
