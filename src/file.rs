use std::{
    io::BufRead,
    path::{Path, PathBuf}, ffi::OsString,
};

use crate::utils::{construct_struct, Env};

/// Retrieves the home path based on the current operating system.
/// 
/// # Returns
/// 
/// The home path as an `OsString` if successful.
/// 
/// # Panics
/// 
/// This function will panic if the current operating system is not supported or if it fails to retrieve the home directory.
pub fn get_home_path() -> OsString {
    let platform = match std::env::consts::OS {
        "windows" => "USERPROFILE",
        "linux" => "HOME",
        "macos" => "HOME",
        _ => {
            println!("{}", std::env::consts::OS);
            panic!("Unsupported platform")
        }
    };

    std::env::var_os(platform).expect("Failed to get home directory")
}

/// Returns the path to the application directory.
/// This can be defined by default in the config file.
/// 
/// # Returns
///
/// The path to the application directory as a `PathBuf` object.
pub fn get_app_dir_path() -> PathBuf {

    let home_path = get_home_path();
    let path = Path::new(&home_path).join(".envn");

    if !&path.exists() {
        let _ = std::fs::create_dir::<_>(&path);
    }

    path
}

/// Represents the configuration file
#[derive(serde::Deserialize, Debug)]
pub struct Config{
    pub base_dir: String,
    pub ask_for_password: bool,
}

/// Returns the default config
fn default_config()->String{
    "base_dir = '~/.envn'\nask_for_password = true".to_string()
}

/// Convert config from string to Config struct
fn convert_config(config: String)->Config{
    let config: Config = toml::from_str(config.as_str()).unwrap_or(toml::from_str(&default_config()).unwrap());
    config
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

/// Retrieves the configuration file.
///
/// # Returns
///
/// The `Config` struct representing the configuration file.
pub fn get_config_file()->Config{
    let home_path = get_home_path();

    // Check if the config file exists, if not create it
    let config_path = Path::new(&home_path).join(".config").join("envn");
    if !&config_path.exists(){
        let _ = std::fs::create_dir::<_>(&config_path);
    }

    let config_file = config_path.join("config.toml");
    if !&config_file.exists(){
        let _ = std::fs::write(&config_file, default_config());
    }

    let config = std::fs::read_to_string(config_path.join("config.toml")).expect("Failed to read config file");
    convert_config(config)
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
/// # Returns
/// 
/// * `true` - If the password was set successfully
/// * `false` - If the password was not set successfully
/// 
/// # Panics
/// * If the password could not be hashed
/// * If the password could not be written to the file
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

/// Loads a file and inserts its contents into the database.
///
/// # Arguments
///
/// * `path` - The path to the file to be loaded.
///
/// # Returns
///
/// A vector of `Env` structs representing the contents of the file.
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
