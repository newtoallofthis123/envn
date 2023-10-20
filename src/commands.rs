use std::{io::Write, path::Path};

use crate::{file::file_exists, utils::DisplayEnv};

pub fn handle_command(cmd: &str, name: Option<String>) {
    match cmd {
        "get" => get_command(name),
        "set" => set_command(),
        "load" => file_command(name),
        "add" => append_env(name),
        "all" => all_command(),
        _ => bunt::println!(
            "{$red}Command Not Found{/$}\nUse {$yellow}envn help{/$} to see available commands"
        ),
    }
}

fn set_command() {
    bunt::println!("The {$yellow}Setter{/$}");

    let key = inquire::Text::new("Secret Name").prompt().unwrap();
    let value = inquire::Text::new("Secret Value").prompt().unwrap();
    let name = inquire::Text::new("The identifier for this secret")
        .prompt()
        .unwrap();

    let env_entry = crate::utils::construct_struct(name, key, value);

    if crate::db::insert_env(env_entry) {
        bunt::println!("{$green}Secret Saved{/$}");
    } else {
        bunt::println!("{$red}Secret Not Saved{/$}");
    }
}

fn get_command(name: Option<String>) {
    bunt::println!("The {$yellow}Getter{/$}");

    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !crate::db::does_exist(&name) {
        bunt::println!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = crate::db::get_by_name(&name);

    let env_entry = env_entry.unwrap();

    let env = crate::utils::decrypt_struct(env_entry);

    crate::utils::display_env(env);
}

fn file_command(filename: Option<String>) {
    bunt::println!("The {$yellow}File{/$}");
    bunt::println!(
        "Pressing enter will take you into add mode. Just press 'quit' to exit add mode"
    );

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

    loop{
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
            bunt::println!("{$red}Secret Not Found{/$}");
            continue;
        }

        let env = crate::db::get_by_name(&to_add).unwrap();
        let final_env = crate::utils::decrypt_struct(env);
        envs_to_write.push(final_env);

        bunt::println!("{$yellow}Secret Added{/$}");

        env_names.remove(env_names.iter().position(|x| *x == to_add).unwrap());
    }

    bunt::println!(
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

    bunt::println!("{$green}File Saved{/$}");
}

fn all_command() {
    bunt::println!("The {$yellow}All{/$}");
    let envs = crate::db::get_all_names();

    for env in envs {
        crate::utils::display_env(env);
    }
}

fn append_env(name: Option<String>) {
    bunt::println!("The {$yellow}Appender{/$}");

    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !crate::db::does_exist(&name) {
        bunt::println!("{$red}Secret Not Found{/$}");
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

    bunt::println!("{$green}Secret Appended{/$}");
}
