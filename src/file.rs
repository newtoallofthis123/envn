use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

//TODO: Implement dynamic path for config file
pub fn get_config_path() -> &'static OsStr {
    // ~/.envn
    let path = std::path::Path::new("/home/noobscience/.envn");
    if !path.exists() {
        let _ = std::fs::create_dir::<_>(path);
    }

    path.as_os_str()
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

pub fn write_file(path: &Path, content: String) -> bool {
    //TODO: Add warning if file exists
    let _ = std::fs::write(path, content);
    true
}

pub fn get_path(joiner: &str) -> PathBuf {
    let path = Path::new(crate::file::get_config_path());
    path.join(joiner).clone()
}

pub fn set_password() -> bool {
    let password = inquire::Password::new("Enter your password 👀")
        .prompt()
        .expect("Failed to get password");
    let key_file = get_path("auth");

    // hash the password
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

    write_file(&key_file, hashed)
}

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
