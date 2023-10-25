use std::{io::Write, path::Path};

use crate::{
    file::{file_exists, self},
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
        "edit" => edit_entry(name),
        "delete" => delete_entry(name),
        "from" => from_file(name),
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

fn edit_entry(entry: Option<String>) {
    let entry = match entry {
        Some(entry) => entry,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !crate::db::does_exist(&entry) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = crate::db::get_by_name(&entry).unwrap();

    let env = crate::utils::decrypt_struct(env_entry);

    print!("The {$yellow}Editor{/$}");

    let key = inquire::Text::new("Secret Name")
        .with_default(env.key.as_str())
        .prompt()
        .unwrap();

    let value = inquire::Text::new("Secret Value")
        .with_default(env.value.as_str())
        .prompt()
        .unwrap();

    let new_env = crate::utils::construct_struct(entry.clone(), key, value);

    //delete old entry
    crate::db::delete_entry_by_name(&entry);

    //insert new entry
    crate::db::insert_env(new_env);

    print!("{$green}Secret Edited{/$}");
}

fn delete_entry(name: Option<String>) {
    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap(),
    };

    if !crate::db::does_exist(&name) {
        print!("{$red}Secret Not Found{/$}");
        return;
    }

    crate::db::delete_entry_by_name(&name);

    print!("{$green}Secret Deleted{/$}");
}

fn from_file(name: Option<String>){
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
        }
        else {
            file_command(Some(name));
            return;
        }
    }

    let envs = crate::file::load_file_to_insert_in_db(Path::new(&name));
    print!("Loaded {$yellow}{}{/$} secrets to memory", envs.len());

    for env in envs {
        if crate::db::does_exist(&env.name) {
            print!("{$red}Secret Already Exists{/$}, use edit instead");
            continue;
        }
        crate::db::insert_env(env);
    }

    print!("{$green}Secrets Saved{/$}");
}