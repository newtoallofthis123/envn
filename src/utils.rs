use aes_gcm::{Aes256Gcm, Key, Nonce};

use crate::{
    db::Entry,
    file::{file_exists, set_password},
};

#[derive(Debug)]
pub struct Env {
    pub name: String,
    pub key: String,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DisplayEnv {
    pub name: String,
    pub key: String,
    pub value: String,
}

fn get_password_from_env() -> Result<String, std::env::VarError> {
    std::env::var("ENVN_PASSWORD")
}

pub fn check_password(password: Option<String>) -> bool {
    let password = match password {
        Some(password) => password,
        None => match get_password_from_env() {
            Ok(password) => password,
            Err(_) => inquire::Password::new("Enter your password 👀")
                .without_confirmation()
                .with_display_mode(inquire::PasswordDisplayMode::Masked)
                .prompt()
                .unwrap(),
        },
    };

    let key_file = crate::file::get_path("auth");

    if !file_exists(&key_file) {
        bunt::println!("{$red}No password file found{/$}");

        let password_confirm = inquire::Confirm::new("Do you want to create a password file?")
            .with_default(true)
            .prompt()
            .unwrap();
        if !password_confirm {
            bunt::println!("{$red}No password file found{/$}");
            return false;
        }

        return set_password();
    }

    let hashed = std::fs::read_to_string(key_file).expect("Failed to read key file");

    bcrypt::verify(password, &hashed).expect("Failed to verify password")
}

pub fn construct_struct(name: String, key: String, value: String) -> Env {
    let (user_key, nonce) = crate::file::get_keys_and_nonce();

    let bytes_of_value = bincode::serialize(&value).expect("Failed to serialize value");
    let encrypted_value = crate::encryption::encrypt(
        Key::<Aes256Gcm>::from_slice(&user_key).to_owned(),
        Nonce::from_slice(&nonce).to_owned(),
        &bytes_of_value,
    );
    Env {
        name,
        key,
        value: encrypted_value,
    }
}

pub fn decrypt_struct(entry: Entry) -> DisplayEnv {
    let (user_key, nonce) = crate::file::get_keys_and_nonce();
    let decrypted_value = crate::encryption::decrypt(
        Key::<Aes256Gcm>::from_slice(&user_key).to_owned(),
        Nonce::from_slice(&nonce).to_owned(),
        entry.value,
    );
    let value = bincode::deserialize(&decrypted_value).expect("Failed to deserialize value");
    DisplayEnv {
        name: entry.name,
        key: entry.key,
        value,
    }
}

pub fn display_env(env: DisplayEnv) {
    bunt::println!("{$blue}\n-----Secret--------{/$}");
    bunt::println!("{$yellow}Name{/$}: {$green}{}{/$}", env.name);
    bunt::println!("{$yellow}Key{/$}: {$green}{}{/$}", env.key);
    bunt::println!("{$yellow}Value{/$}: {$green}{}{/$}", env.value);
}

pub fn display_help(cmd: Option<String>) {
    let cmd = cmd.unwrap_or("all".to_string());
    bunt::println!("The Premium Secret Manager\n");

    bunt::println!("{$underline}Usage:{/$} {$green}evnv{/$} {$yellow}[command] [name]{/$}");

    match cmd.as_str() {
        "all" => {
            bunt::println!("{$blue}Show All{/$} secrets");
            bunt::println!("envn {$green}all{/$} [range]");
        }
        "show" => {
            bunt::println!("{$blue}Show{/$} a secret");
            bunt::println!("envn {$green}show{/$} [name]");
        }
        "add" => {
            bunt::println!("{$blue}Add{/$} a secret");
            bunt::println!("envn {$green}add{/$} [name]");
        }
        "load" => {
            bunt::println!("{$blue}Load{/$} from a file");
            bunt::println!("envn {$green}load{/$} [name]");
        }
        "save" => {
            bunt::println!("{$blue}Save{/$} secret to a file");
            bunt::println!("envn {$green}save{/$} [name]");
        }
        _ => {
            bunt::println!("Available Commands: show, add, load, save, all");
            bunt::println!("Use envn help {$yellow}[command]{/$} to see more info about a command");
        }
    }
}
