use std::path::{Path, PathBuf};

use crate::file::write_file;

pub fn get_path(joiner: &str)->PathBuf{
    let path = Path::new(crate::file::get_config_path());
    let final_path = path.join(joiner);
    return final_path.clone()
}

pub fn set_password()->bool{
    let password = inquire::Password::new("Enter your password 👀").prompt().expect("Failed to get password");
    let key_file = get_path("key");

    // hash the password
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

    return write_file(&key_file, hashed);
}

pub fn check_password()->bool{
    let password = inquire::Password::new("Enter your password 👀").without_confirmation().prompt().expect("Failed to get password");
    let key_file = get_path("key");

    let hashed = std::fs::read_to_string(key_file).expect("Failed to read key file");

    return bcrypt::verify(password, &hashed).expect("Failed to verify password");
}
