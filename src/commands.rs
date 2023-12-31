/// This file deals with the i/o operations of the program
/// No function of this file, expect the handler function, is called directly
/// nor exported to the main file
/// Any new command should be added to this file as well
/// as the handler function
use std::{io::Write, path::Path};

use crate::{
    db::{delete_entry_by_name, does_exist, get_all_names, get_by_name, insert_env},
    file::{self, compress, decompress, file_exists, join_app_path, load_file_to_insert_in_db},
    utils::{construct_struct, decrypt_struct, display_env, get_date_time, DisplayEnv},
};
use bunt::println as print;

/// Handles the command passed in by the user
pub fn handle_command(cmd: &str, name: Option<String>) {
    match cmd {
        "get" => get_command(name),
        "show" => get_command(name),
        "add" => add_command(name),
        "save" => save_command(name),
        "append" => append_env(name),
        "all" => all_command(name),
        "edit" => edit_entry(name),
        "backup" => backup_command(name),
        "restore" => restore_command(name),
        "delete" => delete_entry(name),
        "load" => load_file(name),
        "reset" => reset_command(name),
        _ => print!(
            "{$red}Command Not Found{/$}\nUse {$yellow}envn help{/$} to see available commands"
        ),
    }
}

/// The Add command
fn add_command(name: Option<String>) {
    print!("The {$yellow}Setter{/$}");

    //if name is not provided, ask for it
    let name = match name {
        Some(name) => {
            bunt::println!("{$yellow}Name{/$}: {$green}{}{/$}", name);
            name
        }
        None => inquire::Text::new("The identifier for this secret")
            .prompt()
            .unwrap(),
    };
    let key = inquire::Text::new("Enter the Key").prompt().unwrap();
    let value = inquire::Text::new("Enter the Value").prompt().unwrap();

    // create a struct to store the data
    let env_entry = construct_struct(name, key, value);

    if crate::db::insert_env(env_entry) {
        print!("{$green}Secret Saved{/$}");
    } else {
        print!("{$red}Secret Not Saved{/$}");
    }
}

fn get_command(name: Option<String>) {
    print!("The {$yellow}Getter{/$}");

    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !crate::db::does_exist(&name) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = get_by_name(&name);

    let env_entry = env_entry.unwrap();

    let env = decrypt_struct(env_entry);

    crate::utils::display_env(env);
}

fn save_command(filename: Option<String>) {
    print!("The {$yellow}File{/$}");
    print!("{$yellow}Warning:{/$} This will {$underline}overwrite{/$} any existing file with the same name");
    print!("Pressing enter will take you into add mode. Just press 'quit' to exit add mode");

    let confirm = inquire::Confirm::new("Enter add mode")
        .with_default(true)
        .prompt()
        .unwrap();
    if !confirm {
        return;
    }

    let envs = get_all_names();
    let mut env_names = envs
        .iter()
        .map(|env| env.name.clone())
        .collect::<Vec<String>>();
    env_names.push("quit".to_string());
    let mut envs_to_write: Vec<DisplayEnv> = Vec::new();

    loop {
        if env_names.len() == 1 {
            break;
        }

        let to_add = inquire::Select::new("Select a secret to add", env_names.clone())
            .prompt()
            .unwrap();

        if to_add == "quit" {
            break;
        }

        if !env_names.contains(&to_add) {
            print!("{$red}Secret Not Found{/$}");
            continue;
        }

        let env = get_by_name(&to_add).unwrap();
        let final_env = decrypt_struct(env);
        envs_to_write.push(final_env);

        print!("{$yellow}Secret Added{/$}");

        env_names.remove(env_names.iter().position(|x| *x == to_add).unwrap());
    }

    print!(
        "Loaded {$yellow}{}{/$} secrets to memory",
        envs_to_write.len()
    );

    let filename = match filename {
        Some(filename) => filename,
        None => inquire::Text::new("File Name")
            .with_default(".env")
            .prompt()
            .unwrap(),
    };

    let file = std::fs::File::create(filename).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    for env in envs_to_write {
        //format as key=value
        let line = format!("{}={}\n", env.key, env.value);
        writer.write_all(line.as_bytes()).unwrap();
    }

    print!("{$green}File Saved{/$}");
}

fn all_command(range: Option<String>) {
    print!("The {$yellow}Show{/$}");
    let envs = get_all_names();

    if envs.is_empty() {
        print!("{$red}No Secrets Found{/$}");
        return;
    }

    let range: u8 = match range {
        Some(range) => range.parse().unwrap(),
        None => 0,
    };

    for (i, env) in envs.iter().enumerate() {
        if range != 0 && i as u8 == range {
            break;
        }
        display_env(env.clone());
    }
}

fn append_env(name: Option<String>) {
    print!("The {$yellow}Appender{/$}");

    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !does_exist(&name) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = get_by_name(&name).unwrap();

    let env = decrypt_struct(env_entry);

    if !file_exists(Path::new(".env")) {
        let _ = std::fs::File::create(".env");
    }

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(".env")
        .unwrap();

    let line = format!("{}={}\n", env.key, env.value);

    file.write_all(line.as_bytes()).unwrap();

    print!("{$green}Secret Appended{/$}");
}

fn edit_entry(entry: Option<String>) {
    let entry = match entry {
        Some(entry) => entry,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !does_exist(&entry) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = get_by_name(&entry).unwrap();

    let env = decrypt_struct(env_entry);

    print!("The {$yellow}Editor{/$}");

    let key = inquire::Text::new("Secret Name")
        .with_initial_value(env.key.as_str())
        .prompt()
        .unwrap();

    let value = inquire::Text::new("Secret Value")
        .with_initial_value(env.value.as_str())
        .prompt()
        .unwrap();

    let new_env = construct_struct(entry.clone(), key, value);

    //delete old entry
    delete_entry_by_name(&entry);

    //insert new entry
    insert_env(new_env);

    print!("{$green}Secret Edited{/$}");
}

fn delete_entry(name: Option<String>) {
    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !does_exist(&name) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    let confirmation = inquire::Confirm::new("Delete from file as well?")
        .with_default(true)
        .prompt()
        .unwrap();

    if confirmation {
        delete_entry_by_name(&name);
        print!("{$green}Secret Deleted{/$}");
    } else {
        print!("{$red}Aborted{/$}");
    }
}

fn load_file(name: Option<String>) {
    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("File Name").prompt().unwrap(),
    };

    if !file::file_exists(Path::new(&name)) {
        print!("{$red}File Not Found{/$}");
        let confirm = inquire::Confirm::new("Wanna Load instead?")
            .with_default(true)
            .prompt()
            .unwrap();
        if !confirm {
            return;
        } else {
            save_command(Some(name));
            return;
        }
    }

    let envs = load_file_to_insert_in_db(Path::new(&name));
    print!("Loaded {$yellow}{}{/$} secrets to memory", envs.len());

    for env in envs {
        if does_exist(&env.name) {
            print!("{$red}Secret Already Exists{/$}, use edit instead");
            continue;
        }
        insert_env(env);
    }

    print!("{$green}Secrets Saved{/$}");
}

fn reset_command(command: Option<String>) {
    let cmd = match command {
        Some(cmd) => cmd,
        None => inquire::Select::new("Select a command to reset", vec!["all", "db", "password"])
            .prompt()
            .unwrap()
            .to_string(),
    };

    let auth_file = file::join_app_path("auth");
    let key_file = file::join_app_path("key");
    let nonce_file = file::join_app_path("nonce");
    let db_file = file::join_app_path("env.db");

    match cmd.as_str() {
        "all" => {
            bunt::println!("{$yellow}Warning:{/$} This will {$underline}delete{/$} all your secrets and the password");
            let confirm = inquire::Confirm::new("Are you sure?")
                .with_default(false)
                .prompt()
                .unwrap();
            if !confirm {
                return;
            }
            let _ = std::fs::remove_file(auth_file);
            let _ = std::fs::remove_file(key_file);
            let _ = std::fs::remove_file(nonce_file);
            let _ = std::fs::remove_file(db_file);
            print!("{$green}Reset Complete{/$}");
        }
        "db" => {
            bunt::println!(
                "{$yellow}Warning:{/$} This will {$underline}delete{/$} all your secrets"
            );
            let confirm = inquire::Confirm::new("Are you sure?")
                .with_default(false)
                .prompt()
                .unwrap();
            if !confirm {
                return;
            }
            let _ = std::fs::remove_file(db_file);
            let _ = std::fs::remove_file(key_file);
            let _ = std::fs::remove_file(nonce_file);
            print!("{$green}Reset Complete{/$}");
        }
        "password" => {
            bunt::println!("{$yellow}Warning:{/$} This will {$underline}delete{/$} your password");
            let confirm = inquire::Confirm::new("Are you sure?")
                .with_default(false)
                .prompt()
                .unwrap();
            if !confirm {
                return;
            }
            let _ = std::fs::remove_file(auth_file);
            print!("{$green}Reset Complete{/$}");
        }
        _ => bunt::println!("{$red}Command Not Found{/$}"),
    }
}

fn backup_command(name: Option<String>) {
    let name = match name {
        Some(name) => match name.ends_with(".tar") {
            true => name,
            false => format!("{}.tar", name),
        },
        None => format!("envn_backup_{}.tar", get_date_time()),
    };
    match compress(&name) {
        Ok(_) => bunt::println!("{$green}Backup {$white}{}{/$} Created{/$}", name),
        Err(_) => bunt::println!("{$red}Backup Failed{/$}"),
    }
}

fn restore_command(name: Option<String>) {
    let name = match name {
        Some(name) => match name.ends_with(".tar") {
            true => name,
            false => format!("{}.tar", name),
        },
        None => inquire::Text::new("Backup File Name").prompt().unwrap(),
    };

    if !join_app_path("backups").join(&name).exists() {
        bunt::println!("{$red}Backup File Not Found{/$}");
        return;
    }

    match decompress(&name) {
        Ok(_) => bunt::println!("{$green}Backup restored from {$white}{}{/$}{/$}", name),
        Err(_) => bunt::println!("{$red}Backup Failed{/$}"),
    }
}
