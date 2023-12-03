use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

use crate::utils::{construct_struct, Env};

/// Matches the respectively app dir path for the operating system and
/// creates a app dir directory with the name ".envn" if it does not exist.
/// This directory is used to store the key, nonce, and auth files.
/// The main sqlite database is stored in the same directory
pub fn get_app_dir_path() -> PathBuf {
    let platform = match std::env::consts::OS {
        "windows" => "USERPROFILE",
        "linux" => "HOME",
        "macos" => "HOME",
        _ => {
            println!("{}", std::env::consts::OS);
            panic!("Unsupported platform")
        }
    };

    let home_path = std::env::var_os(platform).expect("Failed to get home directory");
    let path = Path::new(&home_path).join(".envn");

    if !path.clone().exists() {
        let _ = std::fs::create_dir::<_>(path.clone());
    }

    path
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

/// Write a file to the specified path
pub fn write_file(path: &Path, content: String) -> bool {
    //TODO: Add warning if file exists
    let _ = std::fs::write(path, content);
    true
}

/// Join any path to the config path
pub fn get_path(joiner: &str) -> PathBuf {
    let path = crate::file::get_app_dir_path();
    path.join(joiner).clone()
}

/// Sets the password for the database
/// This password is used for the encryption and decryption of the database
/// Also, a hashed version of the password is stored in the auth file
/// The auth algorithm is bcrypt
/// Which is a hashing algorithm that is used to hash passwords and used to 
/// verify the password when the user tries to access the database
pub fn set_password() -> bool {
    let password = inquire::Password::new("Enter your password 👀")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()
        .expect("Failed to get password");
    let key_file = get_path("auth");

    // hash the password
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

    bunt::println!("{$green}Password Set{/$}");
    bunt::println!("Restart the program to use the password");
        
    write_file(&key_file, hashed)
}

/// This function returns a tuple containing two vectors: the keys and the nonce.
/// The keys vector contains a sequence of bytes representing the encryption keys,
/// while the nonce vector contains a sequence of bytes representing the unique
/// number used once in encryption algorithms.
///
/// # Returns
///
/// A tuple `(keys, nonce)` where `keys` is a vector of bytes representing the encryption keys,
/// and `nonce` is a vector of bytes representing the nonce.
pub fn get_keys_and_nonce() -> (Vec<u8>, Vec<u8>) {
    let key_path = get_path("key");
    let nonce_path = get_path("nonce");

    if !file_exists(&key_path) || !file_exists(&nonce_path) {
        let key = crate::encryption::get_key();
        let nonce = crate::encryption::get_nonce();

        //write the bytes to the file
        let _ = std::fs::write(&key_path, key);
        let _ = std::fs::write(&nonce_path, nonce);
    }

    let key = std::fs::read(key_path).expect("Failed to read key file");
    let nonce = std::fs::read(nonce_path).expect("Failed to read nonce file");

    (key, nonce)
}

/// Loads the data from the file and inserts it into the database
pub fn load_file_to_insert_in_db(path: &Path) -> Vec<Env> {
    let mut envs: Vec<Env> = Vec::new();
    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let split: Vec<&str> = line.split('=').collect();
        let key = split[0];
        let value = split[1];
        let env = construct_struct(key.to_lowercase(), key.to_string(), value.to_string());
        envs.push(env);
    }
    envs
}
