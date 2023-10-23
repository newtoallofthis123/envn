use std::{io::Write, path::Path};

use crate::{
    file::{self, file_exists},
    utils::DisplayEnv,
};
use bunt::println as print;

pub fn handle_command(cmd: &str, name: Option<String>) {
    match cmd {
        "get" => get_command(name),
        "set" => set_command(),
        "load" => file_command(name),
        "add" => append_env(name),
        "show" => all_command(),
        _ => print!(
            "{$red}Command Not Found{/$}\nUse {$yellow}envn help{/$} to see available commands"
        ),
    }
}

fn set_command() {
    print!("The {$yellow}Setter{/$}");

    let key = inquire::Text::new("Secret Name").prompt().unwrap();
    let value = inquire::Text::new("Secret Value").prompt().unwrap();
    let name = inquire::Text::new("The identifier for this secret")
        .prompt()
        .unwrap();

    let env_entry = crate::utils::construct_struct(name, key, value);

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

    let env_entry = crate::db::get_by_name(&name);

    let env_entry = env_entry.unwrap();

    let env = crate::utils::decrypt_struct(env_entry);

    crate::utils::display_env(env);
}

fn file_command(filename: Option<String>) {
    if filename.is_some() {
        if !file_exists(Path::new(filename.as_ref().unwrap())) {
            print!("{$red}No .env file found to load/ file path is incorrect{/$}");
        } else {
            let confirm = inquire::Confirm::new("Duplicate keys will be overwritten!!")
                .with_default(true)
                .prompt()
                .unwrap();
            if !confirm {
                return;
            }

            let envs = file::load_file_to_insert_in_db(Path::new(filename.as_ref().unwrap()));
            for env in envs {
                crate::db::insert_env(env);
            }
            print!("{$green}File Loaded{/$}");
        }
    } else {
        print!("The {$yellow}File{/$}");
        print!("Pressing enter will take you into add mode. Just press 'quit' to exit add mode");

        let confirm = inquire::Confirm::new("Enter add mode")
            .with_default(true)
            .prompt()
            .unwrap();
        if !confirm {
            return;
        }

        let envs = crate::db::get_all_names();
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

            let env = crate::db::get_by_name(&to_add).unwrap();
            let final_env = crate::utils::decrypt_struct(env);
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
            None => inquire::Text::new("File Name").prompt().unwrap(),
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
}

fn all_command() {
    print!("The {$yellow}Show{/$}");
    let envs = crate::db::get_all_names();

    if envs.is_empty() {
        print!("{$red}No Secrets Found{/$}");
        return;
    }

    for env in envs {
        crate::utils::display_env(env);
    }
}

fn append_env(name: Option<String>) {
    print!("The {$yellow}Appender{/$}");

    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !crate::db::does_exist(&name) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = crate::db::get_by_name(&name).unwrap();

    let env = crate::utils::decrypt_struct(env_entry);

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
